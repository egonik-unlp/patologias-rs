use crate::reports::repository::ReportsRepository;

pub struct ReportsService {
    repo: ReportsRepository,
}

impl ReportsService {
    fn new(repo: ReportsRepository) -> Self {
        Self { repo }
    }
}
