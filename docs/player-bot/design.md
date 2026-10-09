# player-bot — Design

> Status: rascunho · Criado em 2026-10-08

## 1. Objetivo

Ter bots que se comportam como players **dentro do servidor rust-ro**. Para o resto do jogo, um bot é um `Character` comum: aparece para os outros players, ganha exp, pega drop, morre e renasce. Não existe cliente nem socket do outro lado.

**Nome:** `player-bot`. No código Rust o módulo se chama `player_bot`, porque nomes de módulo não aceitam hífen. O prefixo `PlayerBot` vale para tipos, a tabela e o atcommand.

O [OpenKore](../../../openkore) serve como **referência de comportamento** (o que fazer e em que ordem). O código dele não é reaproveitado.

### Fora do escopo da v1
- Conversar com NPC (scripts, Kafra, quests)
- Party, guild, trade e chat
- Skills além das já implementadas pelo servidor (ver `docs/progress/`)
- Comportamento "humano" sofisticado (rotas planejadas, economia)

## 2. Decisões tomadas

| Tema | Decisão | Motivo |
|---|---|---|
| Onde roda | Módulo dentro do servidor, numa thread própria | Escala melhor que N processos de cliente, e não depende de compatibilidade de pacotes |
| Como o bot age | Gera `GameEvent`s, igual aos request handlers | O servidor já funciona por troca de mensagens, então o resto do jogo não precisa saber que é um bot |
| Como o bot percebe | **Foto** (snapshot) gerada pelo game loop e enviada por canal | Ler `Character.map_view` de outra thread é condição de corrida (o campo é substituído a cada tick em `character_service.rs:1442`) |
| Persistência | Mesmo Postgres dos players, mais uma tabela `player_bot` | Uma só fonte de verdade, reaproveitando os repositórios existentes |
| SpacetimeDB | Descartado | Dividiria os dados entre dois bancos. A velocidade dele vem de manter o estado em memória, e o rust-ro já faz isso |
| Papel do OpenKore | Referência para a lógica de decisão e os nomes de configuração | O código dele é cliente, em Perl, e orientado a pacotes |

## 3. Arquitetura

```
                 ┌──────────────────────────────── servidor rust-ro ────────────────────────────────┐
                 │                                                                                  │
 cliente real ──▶│ request_handler ──GameEvent──┐                                                   │
                 │                              ▼                                                   │
                 │                  ┌───────────────────────┐  MapEvent   ┌───────────────────┐     │
                 │                  │ game_loop (40ms)      │────────────▶│ map_instance_loop │     │
                 │                  │                       │             │ (mobs, drops)     │     │
                 │                  │ a cada N ticks:       │             └───────────────────┘     │
                 │                  │ PlayerBotPerception   │                                       │
                 │                  └───────────┬───────────┘                                       │
                 │              percepção       │      ▲ GameEvent                                  │
                 │              (canal)         ▼      │ (add_to_next_tick)                         │
                 │                  ┌───────────────────────┐                                       │
                 │                  │ player_bot_thread     │  IA: pilha de tarefas por bot         │
                 │                  │ (PlayerBotManager)    │  pathfinding (fora do game loop)      │
                 │                  └───────────────────────┘                                       │
                 └──────────────────────────────────────────────────────────────────────────────────┘
```

Regras:
- **A IA nunca roda dentro do game loop.** O loop tem orçamento de menos de 20ms por tick (ver `CLAUDE.md`). Dentro dele, só a cópia barata da percepção.
- **A thread dos bots nunca lê o `ServerState` diretamente.** Ela só usa a foto recebida e os dados que não mudam depois do boot (células do mapa, banco de itens, banco de skills).
- **A thread dos bots nunca altera estado.** Toda mudança vira `GameEvent` (ou `CharacterMove` na fila de movimento).

### Módulo proposto

```
server/src/server/player_bot/
├── mod.rs              # PlayerBotManager: thread, canais, registro de bots ativos
├── lifecycle.rs        # spawn / despawn / respawn
├── perception.rs       # PlayerBotPerception + montagem da foto (chamado pelo game loop)
├── action.rs           # PlayerBotAction -> GameEvent
├── ai/
│   ├── mod.rs          # ciclo de decisão (prioridades no estilo CoreLogic)
│   ├── task.rs         # pilha de tarefas (no estilo src/Task do OpenKore)
│   └── tasks/          # Route, Attack, SitRest, PickUp, RandomWalk...
├── config.rs           # perfil e configuração por bot (serde)
└── repository.rs       # acesso à tabela `player_bot`
```

## 4. Ciclo de vida de um bot

Hoje um player passa por estas etapas. Cada uma precisa de um equivalente para o bot:

| # | Player real | Onde | Bot |
|---|---|---|---|
| 1 | Login cria a `Session` | `request_handler/login.rs` | `Session` **sem socket**, marcada como bot |
| 2 | `handle_select_char` lê o personagem do banco e chama `insert_character` | `request_handler/char.rs:238` | Extrair essa montagem para uma função reutilizável e chamá-la pelo `lifecycle::spawn` |
| 3 | `handle_enter_game` envia `CharacterJoinGame`, `schedule_warp_to_walkable_cell` e `CharacterInitInventory` | `request_handler/char.rs:308` | Mesmos eventos, sem os pacotes de socket |
| 4 | O cliente avisa que carregou o mapa → `CharacterLoadedFromClientSide` | `request_handler/map.rs:73` | **Obrigatório**: sem `loaded_from_client_side = true`, o game loop ignora o personagem (`game_loop.rs:51` e `:371`) |
| 5 | Morte → o cliente envia `CzRestart` | `request_handler/char.rs:386` | Lógica equivalente, disparada pela IA |
| 6 | Logout → `CharacterLeaveGame` → `disconnect_character` | `server/mod.rs:330` | O mesmo evento; o estado é salvo igual ao de um player |

### Sessão sem socket
`Session.map_server_socket` já é `Option`. Quando é `None`, `get_map_socket_for_char_id` devolve `None`, e a thread de notificação descarta o pacote (`server/mod.rs:464`). Então pacotes endereçados a um bot já somem sem erro.

Mudanças necessárias:
- Adicionar um marcador de bot em `Session` (o `is_simulated` atual tem outra semântica: o debugger usa um socket TCP de loopback real).
- Adicionar `is_player_bot` em `Character`, para que a thread de notificação **não monte buffers** para bots no laço de FOV (`server/mod.rs:511`).
- As contas dos bots continuam existindo em `ragnarok.login`, porque a `Session` é indexada por `account_id`.

## 5. Percepção

O game loop monta, para cada bot que estiver na vez, uma foto pequena e só com cópias:

```rust
// esboço
pub struct PlayerBotPerception {
    pub tick: u128,
    pub char_id: u32,
    pub map: MapInstanceKey,
    pub position: Position,
    pub action: PlayerBotActionState, // Idle / Moving / Attacking / Sitting / Dead (cópia de CharacterAction)
    pub hp: u32, pub max_hp: u32,
    pub sp: u32, pub max_sp: u32,
    pub weight: u32, pub max_weight: u32,
    pub can_act_at: u128,
    pub mobs: Vec<SeenMob>,           // id, mob_id, posição, hp% e alvo, quando conhecido
    pub players: Vec<SeenPlayer>,     // id, posição, é bot?
    pub items: Vec<SeenItem>,         // itens no chão: id, item_id, posição, dono e trava
    pub consumables: Vec<(usize, u32, i16)>, // índice no inventário, item_id, quantidade
}
```

Fontes no estado atual:
- `Character.map_view: HashSet<MapItem>`, atualizado em `CharacterService::load_units_in_fov`
- `ServerState::map_item_snapshot`, `map_item_mob_status` e `map_item_x_y` (`state/server.rs`)
- `Character.status`, `Character.inventory`, `Character.action`

Frequência: **não precisa ser a cada tick**. Um player reage em 200 a 500ms. Os bots são distribuídos entre os ticks (round-robin): com período de 8 ticks (320ms), cada tick processa cerca de 1/8 dos bots. Assim o custo por tick fica proporcional a `bots / 8`.

## 6. Ação

A IA produz `PlayerBotAction`. O `action.rs` traduz cada uma em eventos já existentes:

| PlayerBotAction | Evento | Observação |
|---|---|---|
| `MoveTo(x, y)` | `CharacterMove` na fila de movimento (`add_to_next_movement_tick`) | O caminho é calculado com `path_search_client_side_algorithm` **na thread do bot**, como faz `handle_char_move` (`request_handler/movement.rs`) |
| `Attack(target)` | `CharacterAttack { repeat: true }` | |
| `UseSkill(id, lvl, target)` | `CharacterUseSkill` | Só skills já implementadas |
| `UseItem(index)` | `CharacterUseItem` | |
| `PickUp(map_item_id)` | `CharacterPickUpItem` | ⚠️ grava no banco de forma síncrona, ver §8 |
| `Sit` / `Stand` | `CharacterSit` / `CharacterStand` | |
| `Respawn` | equivalente a `handle_restart` | |
| `Equip(index)` | `CharacterEquipItem` | |

A fila `TasksQueue` usa `Mutex` (`model/tasks_queue.rs`), então a thread do bot pode chamar `Server::add_to_next_tick` com segurança, do mesmo jeito que os request handlers fazem.

## 7. IA

Esta seção cobre a IA básica (M1 a M3). Personalidade, necessidades, objetivos, grupos e relacionamentos estão em [behavior.md](behavior.md).

### Ciclo de decisão
A cada foto recebida, cada bot avalia as regras em ordem de prioridade. A primeira que se aplica decide o que acontece. A ordem é inspirada em `openkore/src/AI/CoreLogic.pm`:

1. Morto → `Respawn`
2. HP abaixo de `useSelf_item_hp` → usar poção
3. HP ou SP abaixo de `sitAuto_*_lower` e nenhum mob atacando → sentar até `sitAuto_*_upper`
4. Sendo atacado → revidar o atacante
5. Item no chão perto e permitido pegar → `PickUp`
6. `attackAuto` ligado e mob válido no FOV → atacar (escolha de alvo em `AI/Attack.pm`)
7. Fora do `lockMap` → rota até o mapa
8. Ocioso → `route_randomWalk`

### Pilha de tarefas
Uma tarefa longa (rota, perseguir um alvo) fica numa pilha, no estilo de `openkore/src/Task/*`. Cada tarefa tem `update(&perception) -> TaskStatus { Running(Option<PlayerBotAction>), Done, Failed }`. Uma prioridade mais alta interrompe a tarefa atual (por exemplo, levar dano durante um `RandomWalk`).

### Configuração por bot
Os nomes seguem `openkore/control/config.txt`, para facilitar consultar a documentação do OpenKore:

```json
{
  "profile": "swordman_leveling",
  "lockMap": "prt_fild08",
  "attackAuto": 2,
  "attackAuto_party": 0,
  "route_randomWalk": true,
  "sitAuto_hp_lower": 40, "sitAuto_hp_upper": 90,
  "sitAuto_sp_lower": 0,  "sitAuto_sp_upper": 0,
  "useSelf_item": [{ "item_id": 501, "hp_below": 50 }],
  "itemsTakeAuto": 1,
  "think_interval_ms": 320
}
```

### Diferença em relação ao OpenKore
O OpenKore é **orientado a eventos de pacote**: ele mantém o próprio estado a partir do que recebe. O bot daqui **consulta a foto**: o servidor é a fonte de verdade, e a IA não guarda cópia do mundo, só a própria pilha de tarefas e algumas memórias curtas (último atacante, alvo atual, mobs a ignorar).

## 8. Persistência

Os bots são contas e personagens normais em `ragnarok.login` e `ragnarok."char"`, mais:

```sql
CREATE TABLE ragnarok.player_bot (
    char_id    INTEGER PRIMARY KEY REFERENCES ragnarok."char"(char_id) ON DELETE CASCADE,
    profile    VARCHAR(32) NOT NULL,
    config     JSONB NOT NULL DEFAULT '{}',
    enabled    BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

- **Boot:** o `PlayerBotManager` carrega os bots com `enabled = true` e dá spawn de forma escalonada, para não travar o boot.
- **Fábrica de bots:** cria a conta e o personagem (classe, atributos, skills e equipamento iniciais), seguindo um perfil. Fica disponível por atcommand (`@playerbot create`) e também em lote.
- **Durante o jogo:** a persistência acontece pelo `PersistenceEvent`, igual à dos players.

### ⚠️ Gargalo conhecido
Algumas operações chamam o Postgres de forma **síncrona dentro do game loop** (`runtime.block_on`):
- `service/character/inventory_service.rs:88`, `:204`, `:262`, `:756` (adicionar e remover item, compra e venda)
- `service/character/character_service.rs` e `service/item_service.rs`

Com muitos bots pegando loot, cada item vira uma ida ao banco no meio do tick. Antes de escalar (marco M4), essas operações devem seguir o padrão "atualiza a memória primeiro, depois manda um `PersistenceEvent`".

## 9. Pontos de contato com o código existente

| Arquivo | Mudança |
|---|---|
| `server/model/session.rs` | Marcador de bot e construtor de sessão sem socket |
| `server/state/character.rs` | Campo `is_player_bot` |
| `server/request_handler/char.rs` | Extrair a montagem do `Character` (`handle_select_char`) e a entrada no jogo (`handle_enter_game`) para funções que o bot também chama |
| `server/game_loop.rs` | No fim da iteração, montar e enviar as `PlayerBotPerception`s dos bots que estão na vez |
| `server/mod.rs` | Iniciar a `player_bot_thread`; ignorar bots no laço de FOV da thread de notificação |
| `server/request_handler/atcommand.rs` | `@playerbot spawn`, `@playerbot despawn`, `@playerbot create`, `@playerbot list` |
| `lib/configuration` | Seção `player_bot` no `config.json` (habilitar, máximo, período de decisão) |
| `db/` | Migração da tabela `player_bot` |

## 10. Marcos

| Marco | Entrega | Critério de pronto |
|---|---|---|
| **M0** | Bot aparece parado | `@playerbot spawn <char>` coloca um personagem salvo no mapa, e um player real o vê |
| **M1** | Bot anda | Random walk dentro do mapa usando `CharacterMove` |
| **M2** | Bot caça | Ataca o mob mais próximo, revida, morre e renasce |
| **M3** | Bot se sustenta | Senta para recuperar HP, usa poção, pega loot, segue a config por bot |
| **M4** | Escala | 200 ou mais bots sem o game loop passar de 20ms; `block_on` removidos dos caminhos quentes |
| **M5** | Persistente | Tabela `player_bot`, fábrica de bots, spawn automático no boot |

### Andamento

- **M0** — implementado em 2026-10-08 (branch `player-bot`), falta validar em jogo com um cliente.
  - `server/src/server/player_bot/lifecycle.rs`: `spawn`, `despawn`, `player_bots`
  - `request_handler/char.rs::load_character`, extraído de `handle_select_char` e usado pelos dois caminhos
  - O game loop marca o bot como carregado sozinho após cada `CharacterChangeMap`, o que cobre spawn, `@warp`, portais e respawn
  - Testes: `server/src/tests/player_bot_lifecycle_test.rs`
  - Seção `player_bot` no `config.json` (`PlayerBotConfig` em `lib/configuration`). Por enquanto só tem `enabled`, que vem `false` por padrão; desligado, `lifecycle::spawn` recusa

Testes: cada marco ganha testes em `server/src/tests/` (a IA pode ser testada pura, com `PlayerBotPerception` montada à mão, sem servidor).

## 11. Questões em aberto

- Quantos bots no máximo, por mapa e no total? Isso define o período de decisão e se uma única `player_bot_thread` basta.
- Bots devem ignorar o "kill steal" (mob já atacado por outro)? Na v1 a sugestão é sim, respeitando `attackAuto_notInTown` e "não atacar mob com outro alvo".
- Como identificar bots para os players (nome, prefixo, nada)?
- `CharacterAction` está sendo refatorado (`docs/state-machine-design.md`). A percepção deve copiar esse enum, não os campos antigos.
- Teleporte (Fly Wing / Butterfly Wing) entra na v1?
- **Limitação conhecida (M0):** `spawn` recusa uma conta que já tem sessão, mas o contrário não é bloqueado. Se um player fizer login na conta de um bot ativo, o `handle_login` substitui a sessão do bot. Até a fábrica de bots (M5) criar contas dedicadas, use contas exclusivas para bots.

## 12. Referências

| OpenKore | Para quê |
|---|---|
| `src/AI/CoreLogic.pm` | Ordem de prioridades do ciclo de decisão |
| `src/AI/Attack.pm` | Escolha de alvo, perseguição, desistência |
| `src/Task/*.pm` (`Route`, `Move`, `UseSkill`, `SitStand`) | Modelo da pilha de tarefas |
| `control/config.txt` | Nomes e semântica das opções |

| rust-ro | Para quê |
|---|---|
| `docs/monster-ai.md` e `map_instance_loop.rs` | Precedente de IA rodando dentro do servidor |
| `docs/state-machine-design.md` | Estados do personagem |
| `debugger/multi_player_simulator.rs` | Tentativa anterior de personagens simulados (usa socket de loopback) |
