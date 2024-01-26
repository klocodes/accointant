use crate::db::transaction::pg_manager::PgTransactionManager;

pub struct TransactionContainer<'a> {
    mgr: PgTransactionManager<'a>,
}

impl<'a> TransactionContainer<'a> {
    pub fn new() -> Self {
        let mgr = PgTransactionManager::new();

        Self { mgr }
    }

    pub fn get_manager(&mut self) -> &mut PgTransactionManager<'a> {
        &mut self.mgr
    }

    pub fn take_manager(mut self) -> PgTransactionManager<'a> {
        self.mgr
    }
}