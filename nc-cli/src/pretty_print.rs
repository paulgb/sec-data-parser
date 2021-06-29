use colored::Colorize;
use sec_data_parser::{Company, CompanyData, Submission, Document, TypedData};

pub trait PrettyPrint {
    fn pretty_print_with_indent(&self, indent: u32);

    fn pretty_print(&self) {
        self.pretty_print_with_indent(0);
    }
}

impl PrettyPrint for (&str, &str) {
    fn pretty_print_with_indent(&self, indent: u32) {
        let (key, value) = *self;

        for _ in 0..indent {
            print!(" ");
        }

        println!("{}: {}", key.green(), value.bright_blue());
    }
}

impl PrettyPrint for CompanyData {
    fn pretty_print_with_indent(&self, indent: u32) {
        PrettyPrint::pretty_print_with_indent(&("Name", self.conformed_name.as_str()), indent + 1);
        PrettyPrint::pretty_print_with_indent(&("CIK", self.cik.as_str()), indent + 1);
    }
}

impl PrettyPrint for Company {
    fn pretty_print_with_indent(&self, indent: u32) {
        if let Some(cd) = &self.company_data {
            cd.pretty_print_with_indent(indent);
        }

        if let Some(cd) = &self.owner_data {
            cd.pretty_print_with_indent(indent);
        }
    }
}

impl PrettyPrint for TypedData {
    fn pretty_print_with_indent(&self, indent: u32) {
        PrettyPrint::pretty_print_with_indent(&("Data Type", self.data_type.to_string().as_str()), indent);
        PrettyPrint::pretty_print_with_indent(&("Data", self.body.to_string().as_str()), indent);
    }
}

impl PrettyPrint for Document {
    fn pretty_print_with_indent(&self, indent: u32) {
        PrettyPrint::pretty_print_with_indent(&("Type", self.doc_type.as_str()), indent + 1);
        PrettyPrint::pretty_print_with_indent(&("Filename", self.filename.as_str()), indent + 1);

        if let Some(description) = &self.description {
            PrettyPrint::pretty_print_with_indent(&("Description", description.as_str()), indent + 1);
        }

        if let Some(body) = &self.body {
            body.pretty_print_with_indent(indent + 1);
        }
    }
}

impl PrettyPrint for Submission {
    fn pretty_print_with_indent(&self, indent: u32) {
        if let Some(filing_date) = self.filing_date {
            PrettyPrint::pretty_print_with_indent(&("Filing Date", filing_date.to_string().as_str()), indent);
        }

        for filer in &self.reporting_owners {
            println!("{}", "Reporting Owner".yellow());
            filer.pretty_print_with_indent(indent)
        }

        for filer in &self.filers {
            println!("{}", "Filer".yellow());
            filer.pretty_print_with_indent(indent)
        }

        for document in &self.documents {
            println!("{}", "Document".yellow());
            document.pretty_print_with_indent(indent)
        }
    }
}