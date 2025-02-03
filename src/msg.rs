use cosmwasm_schema::{cw_serde, QueryResponses};
use andromeda_std::{andr_exec, andr_instantiate, andr_query};

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {% raw %}{{% endraw %}{% unless minimal %}
    pub count: i32,
{% endunless %}}

#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {% raw %}{{% endraw %}{% unless minimal %}
    #[attrs(nonpayable)]
    Increment {},
    // Reset can only be called by the owner and does not accept funds
    #[attrs(restricted, nonpayable)]
    Reset { count: i32 },
{% endunless %}}

#[andr_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {% raw %}{{% endraw %}{% unless minimal %}
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
{% endunless %}}
{% unless minimal %}
// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
{% endunless %}