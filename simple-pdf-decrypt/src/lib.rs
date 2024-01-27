use simple_pdf_parser::Pdf;
use std::cell::RefCell;
use std::rc::Rc;

mod compute_key;

mod decrypt;

#[derive(Default, Clone)]
pub struct KeyManager {
    pub pdf_rc: Rc<RefCell<Pdf>>,
    pub workspace_key: Option<[u8; 16]>,
}

impl KeyManager {
    pub fn new(pdf: &Pdf) -> Self {
        Self {
            pdf_rc: Rc::new(RefCell::new(pdf.clone())),
            workspace_key: None,
        }
    }
}
