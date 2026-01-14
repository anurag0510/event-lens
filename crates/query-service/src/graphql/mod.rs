use elasticsearch::SearchParts;
use serde::Deserialize;
use shared::OrderEvent;

pub struct QueryRoot;

#[derive(Deserialize, Debug)]
struct EsSearchResponse {
    hits: EsHits,
}

#[derive(Deserialize, Debug)]
struct EsHits {
    hits: Vec<EsHit>,
}

#[derive(Deserialize, Debug)]
struct EsHit {
    _source: OrderEvent,
}

#[async_graphql::Object]
impl QueryRoot {
    async fn search_orders(
        &self,
        ctx: &async_graphql::Context<'_>,
        region: Option<String>,
        category: Option<String>,
    ) -> async_graphql::Result<Vec<OrderEvent>> {
        let client = ctx
            .data::<elasticsearch::Elasticsearch>()
            .map_err(|_| "ES Client not found in context")?;

        // 1. Build the Dynamic Query DSL
        let mut must_clauses = vec![];

        if let Some(r) = region {
            must_clauses.push(serde_json::json!({ "match": { "region": r } }));
        }
        if let Some(c) = category {
            must_clauses.push(serde_json::json!({ "match": { "category": c } }));
        }

        let query_body = serde_json::json!({
            "query": {
                "bool": {
                    "must": must_clauses
                }
            },
            "size": 2 // Limit results for performance
        });

        // 2. Execute the Search
        let response = client
            .search(SearchParts::Index(&["orders"]))
            .body(&query_body)
            .send()
            .await?;
        
        let response2 = client
            .search(SearchParts::Index(&["orders"]))
            .body(&query_body)
            .send()
            .await?;

        println!("ES Response Status: {:#}", response2.text().await?);
        // 3. Hydrate Data (Map ES hits back to shared::OrderEvent)
        let results: EsSearchResponse = response.json().await?;
        println!("ES Search Results: {:#?}", results);

        let orders = results.hits.hits.into_iter().map(|h| h._source).collect();

        Ok(orders)
    }
}
