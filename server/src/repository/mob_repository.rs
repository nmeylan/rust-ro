use async_trait::async_trait;
use sqlx::Error;

use crate::repository::model::mob_model::MobModel;
use crate::repository::{MobRepository, PgRepository};

#[async_trait]
impl MobRepository for PgRepository {

    async fn get_all_mobs(&self) -> Result<Vec<MobModel>, Error> {
        sqlx::query_as("SELECT mob_db.*, i1.id as drop1_itemid, i2.id  as drop2_itemid, i3.id as drop3_itemid, i4.id  as drop4_itemid, i5.id  as drop5_itemid, i6.id  as drop6_itemid, i7.id  as drop7_itemid,
       i8.id  as drop8_itemid,i9.id  as drop9_itemid,i10.id  as drop10_itemid,
       imvp1.id  as mvpdrop1_itemid,imvp2.id  as mvpdrop2_itemid,imvp3.id  as mvpdrop3_itemid
       FROM mob_db
         LEFT OUTER JOIN item_db i1 ON  i1.name_aegis = mob_db.drop1_item
         LEFT OUTER JOIN item_db i2 ON  i2.name_aegis = mob_db.drop2_item
         LEFT OUTER JOIN item_db i3 ON  i3.name_aegis = mob_db.drop3_item
         LEFT OUTER JOIN item_db i4 ON  i4.name_aegis = mob_db.drop4_item
         LEFT OUTER JOIN item_db i5 ON  i5.name_aegis = mob_db.drop5_item
         LEFT OUTER JOIN item_db i6 ON  i6.name_aegis = mob_db.drop6_item
         LEFT OUTER JOIN item_db i7 ON  i7.name_aegis = mob_db.drop7_item
         LEFT OUTER JOIN item_db i8 ON  i8.name_aegis = mob_db.drop8_item
         LEFT OUTER JOIN item_db i9 ON  i9.name_aegis = mob_db.drop9_item
         LEFT OUTER JOIN item_db i10 ON  i10.name_aegis = mob_db.drop10_item
         LEFT OUTER JOIN item_db imvp1 ON  imvp1.name_aegis = mob_db.mvpdrop1_item
         LEFT OUTER JOIN item_db imvp2 ON  imvp2.name_aegis = mob_db.mvpdrop2_item
         LEFT OUTER JOIN item_db imvp3 ON  imvp3.name_aegis = mob_db.mvpdrop3_item
;")
            .fetch_all(&self.pool).await
    }
}