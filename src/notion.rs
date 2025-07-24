use notion_client::endpoints::Client as NotionClient;
use notion_client::endpoints::pages::create::{request::CreateAPageRequest};
use notion_client::objects::parent::Parent;
use notion_client::objects::page::{DateOrDateTime, DatePropertyValue, PageProperty, SelectPropertyValue};
use notion_client::objects::rich_text::{RichText, Text};
use crate::{types::{Error, Quote, UserMap}};
use uuid::Uuid;

pub async fn add_quote_to_notion(
    client: &NotionClient,
    database_id: &str,
    quote: &Quote,
    user_map: &UserMap
) -> Result<(), Error> {
    fn make_rich_text_prop(content: String) -> PageProperty {
        PageProperty::RichText { 
            id: Some(Uuid::new_v4().to_string()),
            rich_text: vec![RichText::Text {
                text: Text {
                    content,
                    link: None,
                },
                annotations: Default::default(),
                plain_text: Some(String::new()),
                href: None,
            }], 
        }
    }
    
    let quoted_name = user_map.resolve(&quote.quoted_user.to_string());
    let quoted_by_name = user_map.resolve(&quote.quoted_by.to_string());


    let mut props = std::collections::BTreeMap::new();
    props.insert("Quoted Person".to_string(), 
        PageProperty::MultiSelect { // todo: allow multiple people?
            id: Some(Uuid::new_v4().to_string()), 
            multi_select: vec![SelectPropertyValue{
                name: Some(quoted_name.clone()),
                id: None,
                color: None,
            }], 
        },
    );
    props.insert("Quote".to_string(), make_rich_text_prop(quote.quoted_text.to_string()));
    props.insert("Quoted By".to_string(),
        PageProperty::Select { 
            id: Some(Uuid::new_v4().to_string()), 
            select: Some(SelectPropertyValue {
                name: Some(quoted_by_name.clone()),
                id: None,
                color: None,
            }), 
        },
    );
    props.insert("Date".to_string(), PageProperty::Date { 
        id: Some(Uuid::new_v4().to_string()), 
        date: Some(DatePropertyValue{
            start: Some(DateOrDateTime::DateTime(quote.quote_time.with_timezone(&chrono::Utc))),
            end: None,
            time_zone: None
        }),
    });

    let req = CreateAPageRequest {
        parent: Parent::DatabaseId { database_id: database_id.to_string() },
        properties: props,
        icon: None,
        cover: None,
        children: None,
    };

    let res = client.pages.create_a_page(req).await?;
    println!("Success: logged quote to Notion - {}", res.id);

    Ok(())
}