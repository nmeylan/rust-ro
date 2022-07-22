extern crate proc_macro;

use self::proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};
use syn::__private::Span;

fn question_marks(max: usize) -> String {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(|_| "?".to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[proc_macro_derive(SqlInsert)]
pub fn mysql_insert(input: TokenStream) -> TokenStream {
    mysql_derive_insert(input, "insert".to_string(), "insert".to_string())
}
#[proc_macro_derive(SqlUpsert)]
pub fn mysql_replace(input: TokenStream) -> TokenStream {
    mysql_derive_insert(input, "replace".to_string(), "upsert".to_string())
}

fn mysql_derive_insert(input: TokenStream, insert_keyword: String, function_name: String) -> TokenStream {
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
    let function_name = Ident::new(function_name.as_str(), Span::call_site());

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
                format!("{} into `{}` ( `{}` ) values ({})", #insert_keyword, table, #columns, #values)
            }

            pub async fn #function_name(&self, executor: &sqlx::MySqlPool, table: &str) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
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
