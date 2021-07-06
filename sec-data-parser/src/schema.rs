use crate::document_body::TypedData;
use crate::document_tree::DocumentTree;
use crate::document_tree::DocumentTree::ContainerNode;
use crate::error::Result;
use crate::tag::{ContainerTag, ValueTag};
use crate::types::{parse_bool, parse_date, parse_date_time, MonthDayPair};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FilingValues {
    pub form_type: String,
    pub act: Option<String>,
    pub file_number: Option<String>,
    pub film_number: Option<String>,
}

impl FilingValues {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut form_type = None;
        let mut act = None;
        let mut file_number = None;
        let mut film_number = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::FormType => {
                        assert!(form_type.is_none());
                        form_type = Some(value.clone());
                    }
                    ValueTag::Act => {
                        assert!(act.is_none());
                        act = Some(value.clone());
                    }
                    ValueTag::FileNumber => {
                        assert!(file_number.is_none());
                        file_number = Some(value.clone());
                    }
                    ValueTag::FilmNumber => {
                        assert!(film_number.is_none());
                        film_number = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(FilingValues {
            form_type: form_type.unwrap(),
            act,
            file_number,
            film_number,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompanyData {
    pub conformed_name: String,
    pub cik: String,
    pub irs_number: Option<String>,
    pub state_of_incorporation: Option<String>,
    pub fiscal_year_end: Option<MonthDayPair>,
    pub assigned_sic: Option<String>,
    pub relationship: Option<String>,
}

impl CompanyData {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut conformed_name = None;
        let mut cik = None;
        let mut irs_number = None;
        let mut state_of_incorporation = None;
        let mut fiscal_year_end = None;
        let mut assigned_sic = None;
        let mut relationship = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::ConformedName => {
                        assert!(conformed_name.is_none());
                        conformed_name = Some(value.clone());
                    }
                    ValueTag::Cik => {
                        assert!(cik.is_none());
                        cik = Some(value.clone());
                    }
                    ValueTag::IrsNumber => {
                        assert!(irs_number.is_none());
                        irs_number = Some(value.clone());
                    }
                    ValueTag::StateOfInforporation => {
                        assert!(state_of_incorporation.is_none());
                        state_of_incorporation = Some(value.clone());
                    }
                    ValueTag::FiscalYearEnd => {
                        assert!(fiscal_year_end.is_none());
                        fiscal_year_end = Some(MonthDayPair::parse(value));
                    }
                    ValueTag::AssignedSic => {
                        assert!(assigned_sic.is_none());
                        assigned_sic = Some(value.clone());
                    }
                    ValueTag::Relationship => {
                        assert!(relationship.is_none());
                        relationship = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(CompanyData {
            conformed_name: conformed_name.unwrap(),
            cik: cik.unwrap(),
            irs_number,
            state_of_incorporation,
            fiscal_year_end,
            assigned_sic,
            relationship,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street1: Option<String>,
    pub street2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}

impl Address {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut street1 = None;
        let mut street2 = None;
        let mut city = None;
        let mut state = None;
        let mut zip = None;
        let mut phone = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::Street1 => {
                        assert!(street1.is_none());
                        street1 = Some(value.clone());
                    }
                    ValueTag::Street2 => {
                        assert!(street2.is_none());
                        street2 = Some(value.clone());
                    }
                    ValueTag::City => {
                        assert!(city.is_none());
                        city = Some(value.clone());
                    }
                    ValueTag::State => {
                        assert!(state.is_none());
                        state = Some(value.clone());
                    }
                    ValueTag::Zip => {
                        assert!(zip.is_none());
                        zip = Some(value.clone());
                    }
                    ValueTag::Phone => {
                        assert!(phone.is_none());
                        phone = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Address {
            street1,
            street2,
            city,
            state,
            zip,
            phone,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FormerCompany {
    pub former_conformed_name: String,
    pub date_changed: NaiveDate,
}

impl FormerCompany {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut former_conformed_name = None;
        let mut date_changed = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::FormerConformedName => {
                        assert!(former_conformed_name.is_none());
                        former_conformed_name = Some(value.clone());
                    }
                    ValueTag::DateChanged => {
                        assert!(date_changed.is_none());
                        date_changed = Some(parse_date(value));
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(FormerCompany {
            former_conformed_name: former_conformed_name.unwrap(),
            date_changed: date_changed.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Company {
    pub company_data: Option<CompanyData>,
    pub filing_values: Vec<FilingValues>,
    pub business_address: Option<Address>,
    pub mail_address: Option<Address>,
    pub owner_data: Option<CompanyData>,
    pub former_name: Vec<FormerCompany>,
    pub former_company: Vec<FormerCompany>,
}

impl Company {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut company_data = None;
        let mut filing_values = Vec::new();
        let mut business_address = None;
        let mut mail_address = None;
        let mut owner_data = None;
        let mut former_name = Vec::new();
        let mut former_company = Vec::new();

        for part in parts {
            match &part {
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::CompanyData => {
                        assert!(company_data.is_none());
                        company_data = Some(CompanyData::from_parts(parts)?)
                    }
                    ContainerTag::FilingValues => {
                        filing_values.push(FilingValues::from_parts(parts)?);
                    }
                    ContainerTag::BusinessAddress => {
                        assert!(business_address.is_none());
                        business_address = Some(Address::from_parts(parts)?)
                    }
                    ContainerTag::MailAddress => {
                        assert!(mail_address.is_none());
                        mail_address = Some(Address::from_parts(parts)?)
                    }
                    ContainerTag::FormerCompany => {
                        let _fc = FormerCompany::from_parts(parts)?;
                        former_company.push(_fc);
                    }
                    ContainerTag::OwnerData => {
                        assert!(owner_data.is_none());
                        owner_data = Some(CompanyData::from_parts(parts)?);
                    }
                    ContainerTag::FormerName => {
                        let _fn = FormerCompany::from_parts(parts)?;
                        former_name.push(_fn);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Company {
            company_data,
            filing_values,
            business_address,
            mail_address,
            owner_data,
            former_name,
            former_company,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Document {
    pub doc_type: String,
    pub sequence: u32,
    pub filename: Option<String>,
    pub body: Option<TypedData>,
    pub description: Option<String>,
    pub flawed: bool,
}

impl Document {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut doc_type = None;
        let mut sequence = None;
        let mut filename = None;
        let mut body = None;
        let mut description = None;
        let mut flawed = false;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::Type => {
                        assert!(doc_type.is_none());
                        doc_type = Some(value.clone());
                    }
                    ValueTag::Sequence => {
                        assert!(sequence.is_none());
                        sequence = Some(value.parse().unwrap());
                    }
                    ValueTag::Filename => {
                        assert!(filename.is_none());
                        filename = Some(value.clone());
                    }
                    ValueTag::Description => {
                        assert!(description.is_none());
                        description = Some(value.clone());
                    }
                    ValueTag::Flawed => {
                        flawed = true;
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::TextNode(t) => body = Some(TypedData::from_string(t)),
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Document {
            doc_type: doc_type.unwrap(),
            sequence: sequence.unwrap(),
            filename,
            body,
            description,
            flawed,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassContract {
    pub class_contract_id: String,
    pub class_contract_name: String,
    pub class_contract_ticker_symbol: Option<String>,
}

impl ClassContract {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut class_contract_id = None;
        let mut class_contract_name = None;
        let mut class_contract_ticker_symbol = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::ClassContractId => {
                        assert!(class_contract_id.is_none());
                        class_contract_id = Some(value.clone());
                    }
                    ValueTag::ClassContractName => {
                        assert!(class_contract_name.is_none());
                        class_contract_name = Some(value.clone());
                    }
                    ValueTag::ClassContractTickerSymbol => {
                        assert!(class_contract_ticker_symbol.is_none());
                        class_contract_ticker_symbol = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(ClassContract {
            class_contract_id: class_contract_id.unwrap(),
            class_contract_name: class_contract_name.unwrap(),
            class_contract_ticker_symbol,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Series {
    pub owner_cik: Option<String>,
    pub series_id: String,
    pub series_name: String,
    pub class_contracts: Vec<ClassContract>,
}

impl Series {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut owner_cik = None;
        let mut series_id = None;
        let mut series_name = None;
        let mut class_contracts = Vec::new();

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::OwnerCik => {
                        assert!(owner_cik.is_none());
                        owner_cik = Some(value.clone());
                    }
                    ValueTag::SeriesId => {
                        assert!(series_id.is_none());
                        series_id = Some(value.clone());
                    }
                    ValueTag::SeriesName => {
                        assert!(series_name.is_none());
                        series_name = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::ClassContract => {
                        let class_contract = ClassContract::from_parts(parts)?;
                        class_contracts.push(class_contract);
                    }
                    _ => unimplemented!("{:?}", tag),
                },

                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Series {
            owner_cik,
            series_id: series_id.unwrap(),
            series_name: series_name.unwrap(),
            class_contracts,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcquiringData {
    pub cik: String,
    pub series: Series,
}

impl AcquiringData {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut series = None;
        let mut cik = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(ValueTag::Cik, value) => {
                    assert!(cik.is_none());
                    cik = Some(value.clone());
                }
                DocumentTree::ContainerNode(ContainerTag::Series, parts) => {
                    assert!(series.is_none());
                    series = Some(Series::from_parts(parts)?);
                }
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(AcquiringData {
            series: series.unwrap(),
            cik: cik.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetData {
    pub cik: String,
    pub series: Vec<Series>,
}

impl TargetData {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut series = Vec::new();
        let mut cik = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(ValueTag::Cik, value) => {
                    assert!(cik.is_none());
                    cik = Some(value.clone());
                }
                DocumentTree::ContainerNode(ContainerTag::Series, parts) => {
                    series.push(Series::from_parts(parts)?);
                }
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(TargetData {
            series,
            cik: cik.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Merger {
    pub acquiring_data: AcquiringData,
    pub target_data: Vec<TargetData>,
}

impl Merger {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut acquiring_data = None;
        let mut target_data = Vec::new();

        for part in parts {
            match &part {
                ContainerNode(tag, parts) => match tag {
                    ContainerTag::AcquiringData => {
                        assert!(acquiring_data.is_none());
                        acquiring_data = Some(AcquiringData::from_parts(parts)?)
                    }
                    ContainerTag::TargetData => {
                        target_data.push(TargetData::from_parts(parts)?);
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }
        Ok(Merger {
            acquiring_data: acquiring_data.unwrap(),
            target_data,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewSeriesAndClassesContracts {
    pub owner_cik: Option<String>,
    pub new_series: Vec<Series>,
    pub new_classes_contract: Vec<Series>,
}

impl NewSeriesAndClassesContracts {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut new_series = Vec::new();
        let mut new_classes_contract = Vec::new();
        let mut owner_cik = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(ValueTag::OwnerCik, value) => {
                    assert!(owner_cik.is_none());
                    owner_cik = Some(value.clone());
                }
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::NewSeries => {
                        new_series.push(Series::from_parts(parts)?);
                    }
                    ContainerTag::NewClassesContracts => {
                        new_classes_contract.push(Series::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(NewSeriesAndClassesContracts {
            new_series,
            owner_cik,
            new_classes_contract,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SeriesAndClassesContracts {
    pub series: Vec<Series>,
}

impl SeriesAndClassesContracts {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut series = Vec::new();

        for part in parts {
            match &part {
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::Series => {
                        let s = Series::from_parts(parts)?;
                        series.push(s);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(SeriesAndClassesContracts { series })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MergerSeriesAndClassContracts {
    pub mergers: Vec<Merger>,
}

impl MergerSeriesAndClassContracts {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut mergers = Vec::new();

        for part in parts {
            match &part {
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::Merger => {
                        let merger = Merger::from_parts(parts)?;
                        mergers.push(merger);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(MergerSeriesAndClassContracts { mergers })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SeriesAndClassesContractsData {
    pub existing_series_and_classes_contracts: Option<SeriesAndClassesContracts>,
    pub merger_series_and_classes_contracts: Option<MergerSeriesAndClassContracts>,
    pub new_series_and_classes_contracts: Option<NewSeriesAndClassesContracts>,
}

impl SeriesAndClassesContractsData {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut existing_series_and_classes_contracts = None;
        let mut merger_series_and_classes_contracts = None;
        let mut new_series_and_classes_contracts = None;

        for part in parts {
            match &part {
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::ExistingSeriesAndClassesContracts => {
                        assert!(existing_series_and_classes_contracts.is_none());
                        existing_series_and_classes_contracts =
                            Some(SeriesAndClassesContracts::from_parts(parts)?);
                    }
                    ContainerTag::MergerSeriesAndClassesContracts => {
                        assert!(merger_series_and_classes_contracts.is_none());
                        merger_series_and_classes_contracts =
                            Some(MergerSeriesAndClassContracts::from_parts(parts)?);
                    }
                    ContainerTag::NewSeriesAndClassesContracts => {
                        assert!(new_series_and_classes_contracts.is_none());
                        new_series_and_classes_contracts =
                            Some(NewSeriesAndClassesContracts::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(SeriesAndClassesContractsData {
            existing_series_and_classes_contracts,
            merger_series_and_classes_contracts,
            new_series_and_classes_contracts,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub accession_number: String,
    pub filing_type: String,
    pub items: Vec<String>,
    pub filing_date: NaiveDate,
    pub date_of_filing_date_change: Option<NaiveDate>,
    pub effectiveness_date: Option<NaiveDate>,
    pub period: Option<NaiveDate>,
    pub filers: Vec<Company>,
    pub documents: Vec<Document>,
    pub series_and_classes_contracts_data: Option<SeriesAndClassesContractsData>,
    pub reporting_owners: Vec<Company>,
    pub issuer: Option<Company>,
    pub group_members: Vec<String>,
    pub subject_company: Vec<Company>,
    pub filed_by: Option<Company>,
    pub reference_462b: Option<String>,
    pub is_filer_a_new_registrant: Option<bool>,
    pub is_filer_a_well_known_seasoned_issuer: Option<bool>,
    pub filed_pursuant_to_general_instruction_a2: Option<bool>,
    pub is_fund_24f2_eligible: Option<bool>,
    pub action_date: Option<NaiveDate>,
    pub received_date: Option<NaiveDate>,
    pub ma_i_individual: Option<String>,
    pub abs_rule: Option<String>,
    pub period_start: Option<NaiveDate>,
    pub no_quarterly_activity: Option<bool>,
    pub no_annual_activity: Option<bool>,
    pub abs_asset_class: Option<String>,
    pub depositor_cik: Option<String>,
    pub sponsor_cik: Option<String>,
    pub category: Option<String>,
    pub registered_entity: Option<bool>,
    pub depositor: Option<Company>,
    pub securitizer: Option<Company>,
    pub references_429: Option<String>,
    pub securitizer_cik: Option<String>,
    pub issuing_entity_cik: Option<String>,
    pub issuing_entity_name: Option<String>,
    pub paper: bool,
    pub confirming_copy: bool,
    pub securitizer_file_number: Option<String>,
    pub depositor_file_number: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub private_to_public: bool,
    pub filed_for: Vec<Company>,
    pub public_reference_acc: Option<String>,
    pub public_rel_date: Option<NaiveDate>,
    pub deletion: bool,
    pub correction: bool,
    pub sros: Option<String>,
    pub previous_accession_number: Option<String>,
}

impl Submission {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut accession_number = None;
        let mut filing_type = None;
        let mut public_document_count: usize = 0;
        let mut items = Vec::new();
        let mut filing_date = None;
        let mut date_of_filing_date_change = None;
        let mut effectiveness_date = None;
        let mut filers = Vec::new();
        let mut documents = Vec::new();
        let mut series_and_classes_contracts_data = None;
        let mut period = None;
        let mut reporting_owners = Vec::new();
        let mut issuer = None;
        let mut group_members = Vec::new();
        let mut subject_company = Vec::new();
        let mut filed_by = None;
        let mut reference_462b = None;
        let mut is_filer_a_new_registrant = None;
        let mut is_filer_a_well_known_seasoned_issuer = None;
        let mut filed_pursuant_to_general_instruction_a2 = None;
        let mut is_fund_24f2_eligible = None;
        let mut action_date = None;
        let mut received_date = None;
        let mut ma_i_individual = None;
        let mut abs_rule = None;
        let mut period_start = None;
        let mut no_quarterly_activity = None;
        let mut no_annual_activity = None;
        let mut abs_asset_class = None;
        let mut depositor_cik = None;
        let mut sponsor_cik = None;
        let mut category = None;
        let mut registered_entity = None;
        let mut depositor = None;
        let mut securitizer = None;
        let mut references_429 = None;
        let mut securitizer_cik = None;
        let mut issuing_entity_cik = None;
        let mut issuing_entity_name = None;
        let mut paper = false;
        let mut confirming_copy = false;
        let mut securitizer_file_number = None;
        let mut depositor_file_number = None;
        let mut timestamp = None;
        let mut private_to_public = false;
        let mut filed_for = Vec::new();
        let mut public_reference_acc = None;
        let mut public_rel_date = None;
        let mut deletion = false;
        let mut correction = false;
        let mut sros = None;
        let mut previous_accession_number = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::AccessionNumber => {
                        assert!(accession_number.is_none());
                        accession_number = Some(value.clone());
                    }
                    ValueTag::Type => {
                        assert!(filing_type.is_none());
                        filing_type = Some(value.clone());
                    }
                    ValueTag::PublicDocumentCount => {
                        assert_eq!(0, public_document_count);
                        public_document_count = value.parse().unwrap();
                    }
                    ValueTag::Items => {
                        items.push(value.clone());
                    }
                    ValueTag::FilingDate => {
                        assert!(filing_date.is_none());
                        filing_date = Some(parse_date(value));
                    }
                    ValueTag::DateOfFilingDateChange => {
                        assert!(date_of_filing_date_change.is_none());
                        date_of_filing_date_change = Some(parse_date(value));
                    }
                    ValueTag::EffectivenessDate => {
                        assert!(effectiveness_date.is_none());
                        effectiveness_date = Some(parse_date(value));
                    }
                    ValueTag::Period => {
                        assert!(period.is_none());
                        period = Some(parse_date(value));
                    }
                    ValueTag::GroupMembers => {
                        group_members.push(value.clone());
                    }
                    ValueTag::Reference462B => {
                        assert!(reference_462b.is_none());
                        reference_462b = Some(value.clone());
                    }
                    ValueTag::IsFilerANewRegistrant => {
                        assert!(is_filer_a_new_registrant.is_none());
                        is_filer_a_new_registrant = Some(parse_bool(value));
                    }
                    ValueTag::IsFilerAWellKnownSeasonedIssuer => {
                        assert!(is_filer_a_well_known_seasoned_issuer.is_none());
                        is_filer_a_well_known_seasoned_issuer = Some(parse_bool(value));
                    }
                    ValueTag::FiledPursuantToGeneralInstructionA2 => {
                        assert!(filed_pursuant_to_general_instruction_a2.is_none());
                        filed_pursuant_to_general_instruction_a2 = Some(parse_bool(value));
                    }
                    ValueTag::IsFund24F2Eligible => {
                        assert!(is_fund_24f2_eligible.is_none());
                        is_fund_24f2_eligible = Some(parse_bool(value));
                    }
                    ValueTag::ActionDate => {
                        assert!(action_date.is_none());
                        action_date = Some(parse_date(value));
                    }
                    ValueTag::ReceivedDate => {
                        assert!(received_date.is_none());
                        received_date = Some(parse_date(value));
                    }
                    ValueTag::MaIIndividual => {
                        assert!(ma_i_individual.is_none());
                        ma_i_individual = Some(value.clone());
                    }
                    ValueTag::AbsRule => {
                        assert!(abs_rule.is_none());
                        abs_rule = Some(value.clone());
                    }
                    ValueTag::PeriodStart => {
                        assert!(period_start.is_none());
                        period_start = Some(parse_date(value));
                    }
                    ValueTag::NoQuarterlyActivity => {
                        assert!(no_quarterly_activity.is_none());
                        no_quarterly_activity = Some(parse_bool(value));
                    }
                    ValueTag::NoAnnualActivity => {
                        assert!(no_annual_activity.is_none());
                        no_annual_activity = Some(parse_bool(value));
                    }
                    ValueTag::AbsAssetClass => {
                        assert!(abs_asset_class.is_none());
                        abs_asset_class = Some(value.clone());
                    }
                    ValueTag::DepositorCik => {
                        assert!(depositor_cik.is_none());
                        depositor_cik = Some(value.clone());
                    }
                    ValueTag::SponsorCik => {
                        assert!(sponsor_cik.is_none());
                        sponsor_cik = Some(value.clone());
                    }
                    ValueTag::Category => {
                        assert!(category.is_none());
                        category = Some(value.clone())
                    }
                    ValueTag::RegisteredEntity => {
                        assert!(registered_entity.is_none());
                        registered_entity = Some(parse_bool(value));
                    }
                    ValueTag::References429 => {
                        assert!(references_429.is_none());
                        references_429 = Some(value.clone());
                    }
                    ValueTag::SecuritizerCik => {
                        assert!(securitizer_cik.is_none());
                        securitizer_cik = Some(value.clone());
                    }
                    ValueTag::IssuingEntityCik => {
                        assert!(issuing_entity_cik.is_none());
                        issuing_entity_cik = Some(value.clone());
                    }
                    ValueTag::IssuingEntityName => {
                        assert!(issuing_entity_name.is_none());
                        issuing_entity_name = Some(value.clone());
                    }
                    ValueTag::Paper => {
                        paper = true;
                    }
                    ValueTag::ConfirmingCopy => {
                        confirming_copy = true;
                    }
                    ValueTag::SecuritizerFileNumber => {
                        securitizer_file_number = Some(value.clone());
                    }
                    ValueTag::DepositorFileNumber => {
                        depositor_file_number = Some(value.clone());
                    }
                    ValueTag::Timestamp => {
                        timestamp = Some(parse_date_time(value));
                    }
                    ValueTag::PrivateToPublic => {
                        private_to_public = true;
                    }
                    ValueTag::PublicReferenceAcc => {
                        public_reference_acc = Some(value.clone());
                    }
                    ValueTag::PublicRelDate => {
                        public_rel_date = Some(parse_date(value));
                    }
                    ValueTag::Deletion => {
                        deletion = true;
                    }
                    ValueTag::Correction => {
                        correction = true;
                    }
                    ValueTag::Sros => {
                        sros = Some(value.clone());
                    }
                    ValueTag::PreviousAccessionNumber => {
                        previous_accession_number = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::Filer => {
                        let filer = Company::from_parts(parts)?;
                        filers.push(filer);
                    }
                    ContainerTag::Document => {
                        let document = Document::from_parts(parts)?;
                        documents.push(document);
                    }
                    ContainerTag::SeriesAndClassesContractsData => {
                        assert!(series_and_classes_contracts_data.is_none());
                        series_and_classes_contracts_data =
                            Some(SeriesAndClassesContractsData::from_parts(parts)?);
                    }
                    ContainerTag::ReportingOwner => {
                        let reporting_owner = Company::from_parts(parts)?;
                        reporting_owners.push(reporting_owner);
                    }
                    ContainerTag::Issuer => {
                        assert!(issuer.is_none());
                        issuer = Some(Company::from_parts(parts)?);
                    }
                    ContainerTag::SubjectCompany => {
                        subject_company.push(Company::from_parts(parts)?);
                    }
                    ContainerTag::FiledBy => {
                        // Technically an n=1, but not asserted because at least one historic
                        // filing duplicates it.
                        filed_by = Some(Company::from_parts(parts)?);
                    }
                    ContainerTag::Depositor => {
                        assert!(depositor.is_none());
                        depositor = Some(Company::from_parts(parts)?);
                    }
                    ContainerTag::Securitizer => {
                        assert!(securitizer.is_none());
                        securitizer = Some(Company::from_parts(parts)?);
                    }
                    ContainerTag::FiledFor => {
                        filed_for.push(Company::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Submission {
            accession_number: accession_number.unwrap(),
            filing_type: filing_type.unwrap(),
            items,
            filing_date: filing_date.unwrap(),
            date_of_filing_date_change,
            effectiveness_date,
            filers,
            documents,
            series_and_classes_contracts_data,
            period,
            issuer,
            group_members,
            subject_company,
            filed_by,
            reference_462b,
            is_filer_a_new_registrant,
            is_filer_a_well_known_seasoned_issuer,
            filed_pursuant_to_general_instruction_a2,
            is_fund_24f2_eligible,
            action_date,
            received_date,
            ma_i_individual,
            abs_rule,
            period_start,
            no_quarterly_activity,
            no_annual_activity,
            abs_asset_class,
            depositor_cik,
            sponsor_cik,
            category,
            registered_entity,
            depositor,
            securitizer,
            references_429,
            reporting_owners,
            securitizer_cik,
            issuing_entity_cik,
            issuing_entity_name,
            paper,
            confirming_copy,
            securitizer_file_number,
            depositor_file_number,
            timestamp,
            private_to_public,
            filed_for,
            public_reference_acc,
            public_rel_date,
            deletion,
            correction,
            sros,
            previous_accession_number,
        })
    }
}
