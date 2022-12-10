extern crate proc_macro;

use self::proc_macro::TokenStream;

use quote::quote;

use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};
use syn::__private::Span;

fn question_marks(max: usize) -> String {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(|i| format!("${}", i))
        .collect::<Vec<String>>()
        .join(",")
}

#[proc_macro_derive(SqlInsert)]
pub fn pg_insert(input: TokenStream) -> TokenStream {
    pg_derive_insert(input, "insert".to_string(), "insert into {} ( {} ) values ({})".to_string())
}

fn pg_derive_insert(input: TokenStream, function_name: String, query: String) -> TokenStream {
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
    let columns = format!("{}", fields_list).replace(", ", ",").replace(",\n", ",");
    TokenStream::from(quote! {

        impl #struct_name {

            pub async fn #function_name(&self, executor: &sqlx::PgPool, table: &str) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
                let sql = format!(#query, table, #columns, #values);
                sqlx::query(&sql)
                #(
                    .bind(&self.#field_name2)
                )*
                    .execute(executor).await
            }
        }
    })
}
