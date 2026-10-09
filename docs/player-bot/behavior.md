# player-bot — Comportamento

> Status: rascunho · Criado em 2026-10-09

## 1. Objetivo

O [design.md](design.md) cobre a infraestrutura: thread dos bots, percepção, ações e ciclo de vida. Este documento cobre **como o bot decide o que fazer**: personalidade, necessidades, objetivos, compromissos, grupos e relacionamentos.

A meta é que cada bot tenha um jeito próprio. Alguns vão direto ao objetivo e outros se distraem no caminho. Alguns são generosos e outros são egoístas. Eles lembram uns dos outros, e quem gosta de alguém tem mais chance de fazer grupo com essa pessoa.

### Princípios
- **Separar dados de fórmulas.** A personalidade é dado (números por bot). A urgência e a combinação com a personalidade são fórmulas trocáveis (§9). As fórmulas podem mudar sem mexer no resto.
- **Registrar cada decisão** com as entradas que levaram a ela, para ajustar as fórmulas com base em dados (§9.2).
- **Sobrevivência fica acima de tudo.** Os reflexos (morto, HP baixo, sendo atacado: itens 1 a 4 da §7 do design) não passam pela personalidade.
- **Tudo roda na `player_bot_thread`.** O game loop só copia dados e encaminha estímulos.

## 2. Modelo em camadas

```
Personalidade        traços fixos do bot (arquétipo + variação)
   │
Relacionamentos      o que o bot sente por cada personagem que conhece
   │
Necessidades         faltas persistentes: poções, arma melhor, zeny...
   │  (geram)
Objetivos            intenções ativas na agenda: vender, caçar, encontrar B em X...
   │  (geram)
Plano                pilha de tarefas (design §7): Route, Attack, PickUp...
   │  (gera)
Ação                 PlayerBotAction → GameEvent
```

Ciclo de decisão, a cada percepção recebida:

1. **Reflexos** de sobrevivência
2. **Estímulos novos** (mensagens, cumprimentos, players vistos), que podem gerar objetivos ou desvios
3. Atualização das **necessidades** e das **oportunidades** de contexto
4. **Objetivo do topo** da agenda: avança o plano ou monta um plano novo
5. **Ação**

## 3. Personalidade

### 3.1 Traços

Cada traço vale de 0.0 a 1.0. Um traço ausente vale 0.5 (neutro), para que bots criados antes de um traço existir continuem funcionando.

| Traço | Efeito |
|---|---|
| `Focus` | resistência a desvios do objetivo atual |
| `Sociability` | interesse por estímulos sociais e por estar em grupo |
| `Curiosity` | interesse por eventos novos (MVP, player diferente, lugar novo) |
| `Greed` | quanto pega para si (loot, divisão) |
| `Fairness` | sensibilidade a divisões injustas. Separado de `Greed`: pode existir um egoísta que se acha justo |
| `Caution` | limiares de HP e poção, distância de mobs perigosos |
| `Aggression` | `attackAuto`, perseguir mobs ou desistir deles |
| `Patience` | quanto tempo espera alguém ou alguma coisa |
| `Reliability` | quanto honra uma promessa quando aparece uma distração |
| `Diligence` | tamanho do orçamento de tempo para conveniências quando alguém está esperando (§5.3) |
| `Persistence` | quantas tentativas falhas aguenta antes de desistir de uma busca |
| `Generosity` | disposição para ajudar (zeny, informação, esperar) |
| `Thrift` | quanto aceita gastar |
| `Forgiveness` | velocidade com que o rancor passa (§8.4) |
| `Tolerance` | quanta injustiça aguenta antes de reagir ou sair do grupo |

```rust
pub struct Personality {
    pub archetype: String,
    pub traits: HashMap<Trait, f32>,
    pub seed: u64, // ruído reproduzível por bot
}
```

### 3.2 Arquétipos

A fábrica de bots (M5 do design) escolhe um arquétipo e sorteia cada traço em torno dele, com variação de ±0.15. Os bots ficam reconhecíveis por tipo, mas nenhum fica igual a outro.

```json
{
  "farmer_focado":   { "Focus": 0.85, "Sociability": 0.25, "Diligence": 0.8, "Greed": 0.6 },
  "social_distraido": { "Focus": 0.25, "Sociability": 0.85, "Curiosity": 0.7, "Diligence": 0.3 },
  "altruista":       { "Generosity": 0.85, "Fairness": 0.8, "Greed": 0.2, "Reliability": 0.8 }
}
```

- Os arquétipos ficam em arquivo de configuração: dá para criar um novo sem recompilar.
- A personalidade é sorteada **uma vez** e salva. Ela não muda entre reinícios.

### 3.3 Configuração derivada

Os parâmetros técnicos no estilo OpenKore (design §7) podem ser **derivados** da personalidade em vez de configurados à mão:

| Traço | Parâmetro |
|---|---|
| `Greed` alto | volta para vender com 85% de peso, em vez de 60% |
| `Caution` alto | `useSelf_item hp_below` 60, em vez de 30 |
| `Aggression` | `attackAuto`, persistência na perseguição |

## 4. Necessidades

Uma necessidade é uma **falta que persiste** até ser resolvida. Ela não age sozinha: gera um objetivo quando a intensidade passa de um limite, ou quando o contexto torna a solução barata (§5.2).

```rust
pub struct Need {
    pub kind: NeedKind,  // FreeWeight, Potions, BetterWeapon, BetterArmor, Zeny, ...
    pub intensity: f32,  // 0..1
    pub since: u128,
}
```

Fontes de intensidade:
- **Estado:** peso, quantidade de poções, zeny
- **Desempenho:** médias móveis guardadas pelo bot. Exemplo: tempo médio para matar os mobs do mapa atual comparado com o esperado para o nível → `BetterWeapon`

Referência para `FreeWeight` (regras do pre-re):
- abaixo de 50%: intensidade baixa
- a partir de 50%: a regeneração natural de HP e SP para, então a intensidade sobe
- a partir de 90%: o bot não ataca nem usa skill, e a intensidade fica perto de 1.0

## 5. Objetivos e agenda

### 5.1 Tipos de objetivo

| Tipo | Exemplos | Origem | Regra |
|---|---|---|---|
| **Suporte** | vender loot, comprar poção, descansar | necessidade | pode ser adiado enquanto a urgência deixar, mas nunca é abandonado |
| **Principal** | caçar, upar, quest | personalidade / arquétipo | compete pelo interesse |
| **Conveniência** | "já estou em Prontera, vou ver espadas" | necessidade + oportunidade | só existe enquanto o contexto for barato e houver orçamento |
| **Social** | cumprimentar, responder, encontrar alguém | estímulo / relacionamento | curto |
| **Compartilhado** | caçar com B, procurar uma arma para A | proposta aceita (§7) | compete pelo interesse, mais o peso do compromisso e da afinidade |
| **Compromisso** | `Rendezvous { with, place, deadline }` | promessa feita (§6) | pesa conforme `Reliability` |

```rust
pub struct Goal {
    pub kind: GoalKind,
    pub origin: GoalOrigin,   // Own | Need(NeedKind) | Stimulus(char_id) | Shared(SharedGoalId)
    pub state: GoalState,     // Active | Suspended { since } | Done | Abandoned
    pub plan: TaskStack,
}

pub struct Agenda {
    pub goals: Vec<Goal>,     // o topo é o ativo
    pub detour_started_at: Option<u128>,
}
```

### 5.2 Desvio e oportunidade

Um estímulo ou uma oportunidade pode colocar um objetivo novo no topo da agenda. O objetivo atual passa a `Suspended`.

```
valor = interesse × traço relevante
      + afinidade com quem originou (se houver)
      + intensidade da necessidade × oportunidade(contexto)
      − urgência do objetivo atual × Focus
      − custo do desvio
      + ruído(seed)
```

A fórmula é só ilustrativa. A forma real é decidida depois (§9).

- **Oportunidade** é o custo de resolver uma necessidade agora. Ao lado das lojas, comprar poção é quase de graça. No meio do campo, é caro.
- Uma necessidade não resolvida **continua existindo** e volta a pesar na próxima oportunidade.

### 5.3 Orçamento de tempo

Quando existe um compromisso pendente, a conveniência ganha um orçamento:

```
orçamento = folga até o deadline × f(Diligence) × g(maior intensidade entre as necessidades)
```

- Cada visita a uma loja ou tarefa de conveniência gasta o orçamento.
- A busca para quando o bot acha algo **bom o bastante** (melhor que o atual, não o melhor de todos), quando o orçamento acaba ou depois de N falhas seguidas (N vem de `Persistence`).
- Com `Diligence` baixo, o bot pode estourar o deadline. Quem está esperando reage pela própria `Patience` (§6).

### 5.4 Retomada

O bot volta ao objetivo suspenso quando o desvio termina, quando `now - detour_started_at > Patience` ou quando a urgência do suspenso passa de um limite. **O plano é montado de novo** a partir da posição atual, porque a rota antiga já não vale.

## 6. Compromissos

Uma promessa ("te encontro em X") vira um objetivo `Rendezvous` na agenda, abaixo do que bloqueia o bot:

```
agenda de A:  [SellLoot]                                  ← ativo
              [Rendezvous { with: B, place: X, deadline }]
              [Shared(caçar com B)]                       suspenso
```

- **Quem prometeu:** `Reliability` decide quanto o compromisso resiste a novas distrações.
- **Quem espera:** espera em X (ou caça perto de X) até o `deadline`. Depois disso, `Patience`, `trust` e `affinity` decidem se ele continua esperando ou desiste.
- Cumprir ou quebrar a promessa gera `KeptPromise` / `BrokePromise` (§8.2).

"Pausar com promessa de voltar" é **um padrão só**. Ele serve para o convite inicial, para a pausa de um objetivo compartilhado (§7.3) e para qualquer outro caso.

## 7. Interação entre bots

### 7.1 Mensagens

Os bots da mesma thread conversam por mensagens estruturadas, guardadas numa caixa de entrada de cada bot. Cada mensagem pode também virar uma **fala visível** no chat, só como aparência, para os players reais verem a interação.

```rust
pub enum BotMessage {
    Greet,
    Propose { goal_id: SharedGoalId, kind: GoalKind, place: Option<MapPos> },
    Accept { goal_id: SharedGoalId },
    AcceptLater { goal_id: SharedGoalId, blocked_by: GoalKind, eta_ms: Option<u64> },
    MeetAt { goal_id: SharedGoalId, place: MapPos, wait_until: u128 },
    Decline { goal_id: SharedGoalId },
    NeedSupport { goal_id: SharedGoalId, kind: GoalKind },
    Inform { item_id: u32, place: MapPos, price: u32, seen_at: u128 },
    AskZeny { amount: u32 },
    OfferZeny { amount: u32 },
    Leave { goal_id: SharedGoalId },
}
```

Exemplo do convite:
1. B → A: `Propose(caçar)`
2. A tem interesse (é sociável e já estava caçando), mas tem um suporte urgente: `AcceptLater(blocked_by: SellLoot)`
3. B → A: `MeetAt(X)`
4. A vende, aproveita a oportunidade se o orçamento deixar, e vai até X

### 7.2 Objetivo compartilhado

```rust
pub struct SharedGoal {
    pub id: SharedGoalId,
    pub kind: GoalKind,            // Hunt, SatisfyNeed { of, need }, ...
    pub members: Vec<u32>,
    pub state: SharedGoalState,    // Forming | Active | Paused { waiting_for } | Ended
    pub meeting_point: Option<MapPos>,
    pub ends_at: Option<u128>,     // duração opcional
    pub ledger: HashMap<u32, Contribution>, // loot, kills e zeny por membro
}
```

- Fica no `PlayerBotManager`. Hoje é uma struct comum, porque está numa thread só (ver §11).
- O `ledger` é atualizado pelas diferenças de inventário e zeny na percepção de cada membro, e alimenta o julgamento de justiça (§8.2).
- **A necessidade de um vira objetivo dos dois.** `SatisfyNeed { of: A, need: BetterWeapon }`: B avalia quanto ganha com isso ("se A matar mais rápido, a caçada rende mais para mim também"), além de `Sociability` e afinidade.

### 7.3 Quando um membro precisa de suporte

A enche o inventário e manda `NeedSupport(SellLoot)`. Cada membro decide sozinho:

| Decisão | Quando faz sentido |
|---|---|
| `GoAlong`: vai junto | o próprio peso também está alto, ou é muito sociável, ou tem afinidade alta |
| `WaitAt(place)` | `Aggression` baixa, ou a cidade fica perto |
| `ContinueAlone` e reencontra depois | `Focus` alto, ou peso baixo |
| `Leave` | afinidade baixa ou paciência esgotada |

Seja qual for a escolha, o objetivo compartilhado fica `Paused`, e A empilha `SellLoot` e depois `Rendezvous`.

## 8. Relacionamentos

### 8.1 Modelo

```rust
pub struct Relationship {
    pub other: u32,        // char_id: bot ou player real
    pub affinity: f32,     // -1.0 detesta .. 0.0 neutro .. 1.0 gosta muito
    pub trust: f32,        // cumpre promessas?
    pub familiarity: u32,  // número de interações
    pub last_seen: u128,
    pub last_greeted: u128,
}
```

- **É assimétrico.** Cada bot guarda o próprio lado.
- **Afinidade e confiança são separadas.** A afinidade decide se o bot quer estar junto. A confiança decide se ele espera e se acredita em promessas.

### 8.2 Eventos e julgamento

O mesmo evento gera variações diferentes em cada observador, conforme a personalidade de quem observa:

```rust
pub trait Appraisal {
    fn affinity_delta(&self, event: &SocialEvent, me: &Personality, rel: &Relationship) -> f32;
    fn trust_delta(&self, event: &SocialEvent, me: &Personality, rel: &Relationship) -> f32;
}

pub enum SocialEvent {
    LootShare { mine: u32, theirs: u32 },
    KeptPromise, BrokePromise,
    Helped { kind: HelpKind },   // zeny, informação, esperou, curou
    Greeted, IgnoredGreeting,
    LeftGroup,
    StoleKill,
}
```

Exemplo: um egoísta (`Greed` alto) pega a maior parte do loot. O altruísta (`Fairness` alto) compara a divisão do `ledger` com o que considera justo, e o desvio acima de `Tolerance` vira variação negativa. Cada evento mexe pouco, mas os eventos se acumulam: o grupo funciona por um tempo e depois desanda.

### 8.3 Onde a afinidade pesa

| Decisão | Efeito |
|---|---|
| Aceitar proposta | desloca a curva de aceitação (abaixo) |
| Cumprimentar | quando alguém de quem gosta aparece na percepção, surge um estímulo `Greet` |
| Desviar do objetivo | soma no interesse do estímulo (§5.2) |
| Esperar no encontro | prolonga a paciência, junto com `trust` |
| Ajudar com zeny | `Generosity × affinity` |
| Sair do grupo | `affinity < limiar de tolerância` |

**Curva de aceitação** (forma a decidir, §9):

```
P(aceitar) = sigmoid(k × (interesse + w × affinity − limiar))
```

- neutro: o resultado depende do interesse, e na maioria das vezes o bot aceita
- gosta muito: aceita quase sempre
- não gosta: a chance fica muito baixa, mesmo com uma proposta boa

**Cumprimento:** tem cooldown por par (`last_greeted`). É um objetivo social curto: parar, mandar emote e chat, e opcionalmente fazer uma proposta. Ignorar um cumprimento gera `IgnoredGreeting`.

**Saída do grupo:** usa histerese. O limiar para sair é mais baixo que o limiar para aceitar de novo, para o bot não ficar entrando e saindo. Sair gera `LeftGroup` para quem ficou.

### 8.4 Decaimento

Sem contato, a afinidade volta devagar para o neutro, na velocidade dada por `Forgiveness`. A decidir: se o negativo decai mais devagar que o positivo.

### 8.5 Esquecimento

Para limitar o tamanho, relações neutras, pouco familiares e não vistas há muito tempo são esquecidas. Relações fortes (positivas ou negativas) ficam.

## 9. Fórmulas trocáveis

### 9.1 Interfaces

Cada decisão é um `trait`. O `config.json` escolhe a implementação pelo nome (por exemplo `"detour_policy": "linear_v1"`). Dá para comparar implementações usando uma em parte dos bots e outra no resto.

```rust
pub trait UrgencyModel {
    fn urgency(&self, goal: &Goal, perception: &PlayerBotPerception) -> f32;
}

pub trait NeedModel {
    fn intensity(&self, need: NeedKind, perception: &PlayerBotPerception, memory: &BotMemory) -> f32;
}

pub trait DetourPolicy {
    fn score(&self, urgency: f32, personality: &Personality, stimulus: &Stimulus, rel: Option<&Relationship>) -> f32;
}

pub trait ProposalPolicy {
    fn respond(&self, proposal: &BotMessage, me: &BotMind, from: &Relationship) -> BotMessage;
}

pub trait GroupPolicy {
    fn on_need_support(&self, me: &BotMind, goal: &SharedGoal, from: u32) -> GroupDecision;
    fn should_leave(&self, me: &BotMind, goal: &SharedGoal) -> bool;
}
```

### 9.2 Log de decisões

Toda decisão registra as entradas e o resultado:

```
bot=42 goal=SellLoot urgency=0.63 stimulus=Propose(from=7) affinity=0.4 traits={Focus:0.81,Sociability:0.22} policy=linear_v1 score=-0.31 → AcceptLater
```

É com esse log que se escolhe se um traço soma ou multiplica com a urgência.

## 10. Memória e persistência

| Memória | Conteúdo | Onde |
|---|---|---|
| Personalidade | arquétipo, traços, seed | `player_bot.config` (JSONB) |
| Necessidades | intensidade e `since` | `player_bot.config` |
| Desempenho | médias móveis (tempo para matar por mapa) | `player_bot.config` |
| Mercado | item, lugar, preço, `seen_at`. O bot só sabe o que viu ou ouviu (`Inform`) | `player_bot.config`, ou tabela própria se crescer |
| Relacionamentos | §8.1 | tabela própria |

```sql
CREATE TABLE ragnarok.player_bot_relationship (
    char_id       INTEGER NOT NULL REFERENCES ragnarok."char"(char_id) ON DELETE CASCADE,
    other_char_id INTEGER NOT NULL REFERENCES ragnarok."char"(char_id) ON DELETE CASCADE,
    affinity      REAL NOT NULL DEFAULT 0,
    trust         REAL NOT NULL DEFAULT 0,
    familiarity   INTEGER NOT NULL DEFAULT 0,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (char_id, other_char_id)
);
```

Em memória, tudo fica na `player_bot_thread`. A gravação é em lote via `PersistenceEvent`, nunca no game loop.

## 11. Dependências no servidor

| Recurso | Situação | Caminho |
|---|---|---|
| Vender loot | `inventory_service::character_sell_items` existe | precisa de um evento direto para o bot, validando a distância até o NPC |
| Comprar de NPC | hoje passa pelo script (`script_service::schedule_get_items`) | indexar as lojas de NPC no boot (item, preço, posição) e usar um evento de compra direto |
| Chat e emote | fora da v1 do design | `PlayerBotAction::Say` / `Whisper` / `Emote` |
| Estímulo de chat vindo de player | não existe | o game loop encaminha para a thread dos bots as mensagens cujo ouvinte é um bot |
| Party | não existe no servidor | o objetivo compartilhado funciona sem party, e vira party real quando ela existir |
| Trade (passar zeny) | não existe no servidor | `OfferZeny` depende disso |
| Vending (lojas de player) | não existe no servidor | a memória de mercado começa só com lojas de NPC |
| Várias threads de bots | hoje há uma só | o `SharedGoal` e os relacionamentos entre threads passariam a exigir troca de mensagens. Não espalhar acesso direto ao `SharedGoal` pelo código |
| Eventos sobre players reais | não existe | o game loop precisaria avisar quando alguém rouba um kill ou pega o loot de um bot |

## 12. Marcos

Estes marcos vêm depois do M3 do design.

| Marco | Entrega | Critério de pronto |
|---|---|---|
| **B1** | Agenda e objetivos de suporte | O bot caça, enche o inventário, vai vender e volta para o mapa de caça |
| **B2** | Personalidade e arquétipos | A fábrica sorteia traços. Os parâmetros técnicos derivam deles. As decisões vão para o log |
| **B3** | Necessidades e oportunidade | Ao vender, o bot também compra poção ou equipamento se compensar, respeitando o orçamento |
| **B4** | Relacionamentos | Afinidade e confiança persistidas. Cumprimento com cooldown |
| **B5** | Propostas e compromissos | Convite, `AcceptLater`, `MeetAt` e encontro no ponto X |
| **B6** | Objetivo compartilhado | Caçar juntos, `NeedSupport`, `ledger`, saída por afinidade |
| **B7** | Ajuda e informação | `Inform`, `SatisfyNeed` compartilhado. `OfferZeny` quando houver trade |

Testes: agenda, necessidades, políticas e julgamento são lógica pura. Dá para testá-los com percepção montada à mão, mensagens simuladas e RNG com seed fixa, sem servidor.

## 13. A decidir

- Forma de cada fórmula (§9): o que soma e o que multiplica, como calcular a urgência e a intensidade das necessidades.
- Duração de um objetivo compartilhado: fixa, por interesse ou até alguém sair?
- Quando um membro sai do grupo, além da afinidade.
- Decaimento da afinidade: velocidade, e se o negativo decai mais devagar.
- Valores de `k`, `w` e do limiar na curva de aceitação.
- Lista final de traços e de arquétipos.
- Conteúdo das falas: templates ou LLM. Um LLM teria que ser assíncrono, com limite de chamadas, e não pode bloquear a thread dos bots.
- Relacionamento com players reais: quais eventos observar e como.
