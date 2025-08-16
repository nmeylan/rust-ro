Handle persistence of character buff when character logout (or server is closing).

# Context

Character state is already updated in database when character disconnect or when the server is closing, this is achieved by `character_service` `save_characters_state` function.

When a character receive a buff (from a supportive skill or consumable), it adds a `TemporaryStatusBonus` to `Character` `Status`.

# Goal
- Create new `character_save_temporary_bonus` in `server/src/repository/character_repository.rs` to persist character bonuses
- Update `save_characters_state` function to save character temporary bonuses.

# Guidance
read following files, they contains definition and implementation of bonus structure and related enums/flag
- `lib/models/src/status.rs`
- `lib/models/src/status_bonus.rs`

the table to store temporary bonuses is defined as follow:
```sql
CREATE TABLE ragnarok.sc_data (
                                  account_id INTEGER NOT NULL,
                                  char_id INTEGER NOT NULL,
                                  type integer NOT NULL,
                                  tick INTEGER NOT NULL,
                                  val1 integer DEFAULT 0 NOT NULL,
                                  val2 integer DEFAULT 0 NOT NULL,
                                  val3 integer DEFAULT 0 NOT NULL,
                                  val4 integer DEFAULT 0 NOT NULL
);
```
- type: should contains bonus type
- val1: the first value of bonus if any
- val2: the second value of bonus if any
- val3: the third value of bonus if any
- val4: the fourth value of bonus if any