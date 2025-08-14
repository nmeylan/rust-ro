Handle packet PacketCzReqItemcomposition whith following structure
```rust
#[derive(Clone)]
pub struct PacketCzReqItemcomposition {
    pub raw: Vec<u8>,
    pub packet_id: i16,
    pub packet_id_raw: [u8; 2],
    pub card_index: i16,
    pub card_index_raw: [u8; 2],
    pub equip_index: i16,
    pub equip_index_raw: [u8; 2],
}
```
- `card_index` is the index of the card in the inventory
- `equip_index` is the index of the equipement in the inventory

The business logic implementation, to be written in `inventory_service` is this one:
- Enqueue a new Persistence Task, slot card
- Handle the new Persistence Task in the persistence event loop
- Implement sql in inventory repository, don't forget to define the new function in the trait `InventoryRepository`
- Answer to client with packet `PacketZcAckItemcomposition`
- Write unit test for the new service function, use mock for repository layer as already done in some test case like `test_add_items_in_inventory_should_add_items_in_memory_save_added_item_in_database` in `server/src/tests/inventory_service_test.rs`

Guidance:
- Read how repositories are implemented
- Read how packet `PacketCzReqItemcompositionList` is implemented the entry point is `server/src/server/request_handler/mod.rs`

# Feedback 1
I forgot to give you sql to write in inventory repository.
First rename `slot_card` into `character_slot_card`

The SQL should within a same transaction:
- update inventory item in `inventory` table, to add card. An item can have multiple card 1-4, if `card0` column is null, then set in `card0` with card item id, else if `card1` is null then insert in `card1`, and so on.
- if all card slots have already a value then it is unexpected and the transaction must be rollbacked. and a negative ack should be send to the client.
- delete card_item from inventory

Thus the `character_slot_card` will probably need: `char_id: u32`, `card_inventory_item: InventoryItemModel`, `equipment_inventory_item: InventoryItemModel`, both card and equiment should be retrieved from character inventory, thus update inventory_service implementation.

Table `inventory` structure is following:
- `card0`: slot for first card
- `card1`: slot for second card
- `card2`: slot for third card
- `card3`: slot for fourth card
- `char_id`: character who one the item in his inventory
- `id`: id of the inventory id
- `nameid`: id of the item, you won't need it but just you are not confused