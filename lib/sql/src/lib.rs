extern crate proc_macro;

use self::proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

fn question_marks(max: usize) -> String {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(|_| "?".to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[proc_macro_derive(SqlInsert)]
pub fn derive_from_struct_sqlite(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
                         fields: Fields::Named(fields),
                         ..
                     }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    // Attributes -> field names
    let field_name = fields.iter().map(|field| &field.ident);
    let field_name2 = fields.iter().map(|field| &field.ident);

    let struct_name = &input.ident;

    let field_length = field_name.len();

    let values = question_marks(field_length);

    let fields_list = quote! {
        #(#field_name),*
    };
    let columns = format!("{}", fields_list)
        .replace(", ", "`,`")
        .replace(",\n", "`,`");
    TokenStream::from(quote! {

        impl #struct_name {
            fn insert_query(&self, table: &str) -> String {
                format!("insert into `{}` ( `{}` ) values ({})", table, #columns, #values)
            }

            pub async fn insert(&self, executor: &sqlx::MySqlPool, table: &str) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
                let sql = self.insert_query(table);
                sqlx::query(&sql)
                #(
                    .bind(&self.#field_name2)
                )*
                    .execute(executor).await
            }
        }
    })
}
