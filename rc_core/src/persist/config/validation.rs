#[derive(Debug, Default)]
pub struct ValidationInfo {
    errors: Vec<ValidationMessage>,
    warnings: Vec<ValidationMessage>,
    infos: Vec<ValidationMessage>,
    context: Vec<String>,
}

impl ValidationInfo {
    pub fn error(&mut self, mut msg: ValidationMessage) {
        self.prepend_context_path(&mut msg);
        self.errors.push(msg);
    }

    pub fn warn(&mut self, mut msg: ValidationMessage) {
        self.prepend_context_path(&mut msg);
        self.warnings.push(msg);
    }

    pub fn info(&mut self, mut msg: ValidationMessage) {
        self.prepend_context_path(&mut msg);
        self.infos.push(msg);
    }

    pub fn push_context(&mut self, path: &str) {
        self.context.push(path.to_owned());
    }

    pub fn pop_context(&mut self) -> String {
        self.context.pop().expect("Bad validation context state")
    }

    fn prepend_context_path(&self, msg: &mut ValidationMessage) {
        let mut new_path = self.context.clone();
        new_path.append(&mut msg.path);
        msg.path = new_path;
    }

    pub(super) fn print_messages(&self) {
        const ERR_LABEL: &str = "ERROR";
        const WARN_LABEL: &str = "WARN";
        const INFO_LABEL: &str = "INFO";
        Self::print_infos(&self.infos, INFO_LABEL);
        Self::print_infos(&self.warnings, WARN_LABEL);
        Self::print_infos(&self.errors, ERR_LABEL);
    }

    fn print_infos(infos: &[ValidationMessage], label: &str) {
        for info in infos {
            println!("[{}]{}", label, info.display());
        }
    }

    pub(super) fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

#[derive(Debug)]
pub struct ValidationMessage {
    pub path: Vec<String>,
    pub message: String,
}

impl ValidationMessage {
    pub fn new(path: Vec<String>, message: String) -> Self {
        Self {
            path,
            message,
        }
    }

    fn display(&self) -> String {
        let path = if self.path.is_empty() {
            "<root>".to_owned()
        } else {
            self.path.join(".")
        };
        format!("{}: {}", path, self.message)
    }
}

pub trait SelfValidator {
    type Context: ?Sized;
    fn validate(&self, info: &mut ValidationInfo, ctx: &Self::Context) -> bool;

    fn validate_in(&self, info: &mut ValidationInfo, ctx: &Self::Context, path: &str) -> bool {
        info.push_context(path);
        let res = self.validate(info, ctx);
        assert_eq!(info.pop_context(), path);
        res
    }
}
