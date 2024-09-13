use juniper::{EmptySubscription, FieldResult, GraphQLInputObject, RootNode};
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A loan")]
struct Loan {
    pub id: i32,
    pub user_id: i32,
    pub loan_type_id: i32,
    pub amount: f64,
    pub currency: String,
    pub term_months: i32,
    pub interest_rate: f64,
    pub monthly_payment: f64,
    pub balance: f64,
    pub status: String,
    pub start_date: String,
    pub end_date: String
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A loan")]
struct NewLoan {
    pub user_id: i32,
    pub loan_type_id: i32,
    pub amount: f64,
    pub currency: String,
    pub term_months: i32,
    pub interest_rate: f64,
    pub monthly_payment: f64,
    pub balance: f64,
    pub status: String,
    pub start_date: String,
    pub end_date: String
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn loan(_id: String) -> FieldResult<Loan> {
        Ok(Loan {
            id: 1,
            user_id: 12345,
            loan_type_id: 1,
            amount: 15000.0,
            currency: "USD".to_owned(),
            term_months: 36,
            interest_rate: 5.5,
            monthly_payment: 452.12,
            balance: 13500.0,
            status: "active".to_owned(),
            start_date: "2024-09-04T00:00:00Z".to_owned(),
            end_date: "2027-09-04T00:00:00Z".to_owned()
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_loan(new_loan: NewLoan) -> FieldResult<Loan> {
        Ok(Loan {
            id: 1,
            user_id: new_loan.user_id,
            loan_type_id: new_loan.loan_type_id,
            amount: new_loan.amount,
            currency: new_loan.currency,
            term_months: new_loan.term_months,
            interest_rate: new_loan.interest_rate,
            monthly_payment: new_loan.monthly_payment,
            balance: new_loan.balance,
            status: new_loan.status,
            start_date: new_loan.start_date,
            end_date: new_loan.end_date
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}