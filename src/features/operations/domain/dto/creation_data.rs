use uuid::Uuid;

pub struct CreationData {
    account_id: Uuid,
    kind: String,
    user_id: Uuid,
    category_id: Option<Uuid>,
    category_name: String,
    amount: f64,
    currency: String,
    currency_amount: f64,
    rate: f64,
    label: String,
    tags: Vec<TagData>,
}

pub struct TagData {
    id: Option<Uuid>,
    name: String,
}

impl CreationData {
    pub fn new(
        account_id: Uuid,
        kind: String,
        user_id: Uuid,
        category_id: Option<Uuid>,
        category_name: String,
        amount: f64,
        currency: String,
        currency_amount: f64,
        rate: f64,
        label: String,
        tags: Vec<TagData>,
    ) -> Self {
        Self {
            account_id,
            kind,
            user_id,
            category_id,
            category_name,
            amount,
            currency,
            currency_amount,
            rate,
            label,
            tags,
        }
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn category_id(&self) -> &Option<Uuid> {
        &self.category_id
    }

    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn currency_amount(&self) -> f64 {
        self.currency_amount
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn tags(&self) -> &Vec<TagData> {
        &self.tags
    }
}

impl TagData {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Option<Uuid> {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}