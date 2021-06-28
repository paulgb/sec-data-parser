use crate::document_tree::DocumentTree;
use crate::document_tree::DocumentTree::ContainerNode;
use crate::error::Result;
use crate::tag::{ContainerTag, ValueTag};
use chrono::NaiveDate;
use crate::types::{MonthDayPair, parse_bool};

const DATE_FORMAT: &str = "%Y%m%d";


pub struct FilingValues {
    form_type: String,
    act: Option<String>,
    file_number: Option<String>,
    film_number: Option<String>,
}

impl FilingValues {
    fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut form_type = None;
        let mut act = None;
        let mut file_number = None;
        let mut film_number = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::FormType => {
                        form_type = Some(value.clone());
                    }
                    ValueTag::Act => {
                        act = Some(value.clone());
                    }
                    ValueTag::FileNumber => {
                        file_number = Some(value.clone());
                    }
                    ValueTag::FilmNumber => {
                        film_number = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    _ => unimplemented!("{:?}", tag),
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

pub struct CompanyData {
    conformed_name: String,
    cik: String,
    irs_number: Option<String>,
    state_of_incorporation: Option<String>,
    fiscal_year_end: Option<MonthDayPair>,
    assigned_sic: Option<String>,
}

impl CompanyData {
    fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut conformed_name = None;
        let mut cik = None;
        let mut irs_number = None;
        let mut state_of_incorporation = None;
        let mut fiscal_year_end = None;
        let mut assigned_sic = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::ConformedName => {
                        conformed_name = Some(value.clone());
                    }
                    ValueTag::Cik => {
                        cik = Some(value.clone());
                    }
                    ValueTag::IrsNumber => {
                        irs_number = Some(value.clone());
                    }
                    ValueTag::StateOfInforporation => {
                        state_of_incorporation = Some(value.clone());
                    }
                    ValueTag::FiscalYearEnd => {
                        fiscal_year_end = Some(MonthDayPair::parse(value));
                    }
                    ValueTag::AssignedSic => {
                        assigned_sic = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    _ => unimplemented!("{:?}", tag),
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
        })
    }
}

pub struct Address {
    street1: Option<String>,
    street2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    phone: Option<String>,
}

impl Address {
    fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
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
                        street1 = Some(value.clone());
                    }
                    ValueTag::Street2 => {
                        street2 = Some(value.clone());
                    }
                    ValueTag::City => {
                        city = Some(value.clone());
                    }
                    ValueTag::State => {
                        state = Some(value.clone());
                    }
                    ValueTag::Zip => {
                        zip = Some(value.clone());
                    }
                    ValueTag::Phone => {
                        phone = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    _ => unimplemented!("{:?}", tag),
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

pub struct FormerCompany {
    former_conformed_name: String,
    date_changed: NaiveDate,
}

impl FormerCompany {
    fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut former_conformed_name = None;
        let mut date_changed = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::FormerConformedName => {
                        former_conformed_name = Some(value.clone());
                    }
                    ValueTag::DateChanged => {
                        date_changed = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    _ => unimplemented!("{:?}", tag),
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

pub struct Filer {
    company_data: Option<CompanyData>,
    filing_values: Option<FilingValues>,
    business_address: Option<Address>,
    mail_address: Option<Address>,
    owner_data: Option<CompanyData>,
    former_name: Option<FormerCompany>,
}

impl Filer {
    fn from_parts(parts: &Vec<DocumentTree>) -> Result<Filer> {
        let mut company_data = None;
        let mut filing_values = None;
        let mut business_address = None;
        let mut mail_address = None;
        let mut former_company = None;
        let mut owner_data = None;
        let mut former_name = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::CompanyData => {
                        company_data = Some(CompanyData::from_parts(parts)?)
                    }
                    ContainerTag::FilingValues => {
                        filing_values = Some(FilingValues::from_parts(parts)?)
                    }
                    ContainerTag::BusinessAddress => {
                        business_address = Some(Address::from_parts(parts)?)
                    }
                    ContainerTag::MailAddress => mail_address = Some(Address::from_parts(parts)?),
                    ContainerTag::FormerCompany => {
                        former_company = Some(FormerCompany::from_parts(parts)?)
                    }
                    ContainerTag::OwnerData => {
                        owner_data = Some(CompanyData::from_parts(parts)?);
                    }
                    ContainerTag::FormerName => {
                        former_name = Some(FormerCompany::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Filer {
            company_data,
            filing_values,
            business_address,
            mail_address,
            owner_data,
            former_name,
        })
    }
}

pub struct Document {
    doc_type: String,
    sequence: u32,
    filename: String,
    text: String, // TODO: parse
    description: Option<String>,
}

impl Document {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut doc_type = None;
        let mut sequence = None;
        let mut filename = None;
        let mut text = None;
        let mut description = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::Type => {
                        doc_type = Some(value.clone());
                    }
                    ValueTag::Sequence => {
                        sequence = Some(value.parse().unwrap());
                    }
                    ValueTag::Filename => {
                        filename = Some(value.clone());
                    }
                    ValueTag::Description => {
                        description = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    _ => unimplemented!("{:?}", tag),
                },
                DocumentTree::TextNode(t) => text = Some(t.clone()),
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Document {
            doc_type: doc_type.unwrap(),
            sequence: sequence.unwrap(),
            filename: filename.unwrap(),
            text: text.unwrap(),
            description,
        })
    }
}

pub struct ClassContract {
    class_contract_id: String,
    class_contract_name: String,
    class_contract_ticker_symbol: Option<String>,
}

impl ClassContract {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut class_contract_id = None;
        let mut class_contract_name = None;
        let mut class_contract_ticker_symbol = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::ClassContractId => {
                        class_contract_id = Some(value.clone());
                    }
                    ValueTag::ClassContractName => {
                        class_contract_name = Some(value.clone());
                    }
                    ValueTag::ClassContractTickerSymbol => {
                        class_contract_ticker_symbol = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    _ => unimplemented!("{:?}", tag),
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

pub struct Series {
    owner_cik: Option<String>,
    series_id: String,
    series_name: String,
    class_contract: ClassContract,
}

impl Series {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut owner_cik = None;
        let mut series_id = None;
        let mut series_name = None;
        let mut class_contract = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::OwnerCik => {
                        owner_cik = Some(value.clone());
                    }
                    ValueTag::SeriesId => {
                        series_id = Some(value.clone());
                    }
                    ValueTag::SeriesName => {
                        series_name = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::ClassContract => {
                        class_contract = Some(ClassContract::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },

                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Series {
            owner_cik: owner_cik,
            series_id: series_id.unwrap(),
            series_name: series_name.unwrap(),
            class_contract: class_contract.unwrap(),
        })
    }
}

pub struct SeriesAndCik {
    cik: String,
    series: Series,
}

impl SeriesAndCik {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut series = None;
        let mut cik = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::Cik => {
                        cik = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                }
                DocumentTree::ContainerNode(tag, parts) => {
                    match tag {
                        ContainerTag::Series => {
                            series = Some(Series::from_parts(parts)?);
                        }
                        _ => panic!("Unexpected: {:?}", &part),
                    }
                }

                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(SeriesAndCik {
            series: series.unwrap(),
            cik: cik.unwrap(),
        })
    }
}

pub struct Merger {
    acquiring_data: SeriesAndCik,
    target_data: SeriesAndCik,
}

impl Merger {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut acquiring_data = None;
        let mut target_data = None;

        for part in parts {
            match &part {
                ContainerNode(tag, parts) => {
                    match tag {
                        ContainerTag::AcquiringData => {
                            acquiring_data = Some(SeriesAndCik::from_parts(parts)?)
                        }
                        ContainerTag::TargetData => {
                            target_data = Some(SeriesAndCik::from_parts(parts)?)
                        }
                        _ => panic!("Unexpected: {:?}", &part),
                    }
                }
                _ => panic!("Unexpected: {:?}", &part),
            }
        }
        Ok(Merger {
            acquiring_data: acquiring_data.unwrap(),
            target_data: target_data.unwrap(),
        })
    }
}

pub struct SeriesAndClassesContracts {
    series: Series,
}

impl SeriesAndClassesContracts {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut series = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::Series => {
                        series = Some(Series::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(SeriesAndClassesContracts {
            series: series.unwrap(),
        })
    }
}

pub struct MergerSeriesAndClassContracts {
    mergers: Vec<Merger>,
}

impl MergerSeriesAndClassContracts {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut mergers = Vec::new();

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    _ => panic!("Unexpected: {:?}", &part),
                },
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

        Ok(MergerSeriesAndClassContracts {
            mergers
        })
    }
}

pub struct SeriesAndClassesContractsData {
    existing_series_and_classes_contracts: Option<SeriesAndClassesContracts>,
    merger_series_and_classes_contracts: Option<MergerSeriesAndClassContracts>,
}

impl SeriesAndClassesContractsData {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut existing_series_and_classes_contracts = None;
        let mut merger_series_and_classes_contracts = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::ExistingSeriesAndClassesContracts => {
                        existing_series_and_classes_contracts = Some(SeriesAndClassesContracts::from_parts(parts)?);
                    }
                    ContainerTag::MergerSeriesAndClassesContracts => {
                        merger_series_and_classes_contracts = Some(MergerSeriesAndClassContracts::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(SeriesAndClassesContractsData {
            existing_series_and_classes_contracts,
            merger_series_and_classes_contracts,
        })
    }
}

pub struct Submission {
    accession_number: Option<String>,
    filing_type: Option<String>,
    items: Option<String>,
    filing_date: Option<NaiveDate>,
    date_of_filing_date_change: Option<NaiveDate>,
    effectiveness_date: Option<NaiveDate>,
    period: Option<NaiveDate>,
    filer: Option<Filer>,
    document: Option<Document>,
    series_and_classes_contracts_data: Option<SeriesAndClassesContractsData>,
    reporting_owner: Option<Filer>,
    issuer: Option<Filer>,
    group_members: Option<String>,
    subject_company: Option<Filer>,
    filed_by: Option<Filer>,
    reference_462b: Option<String>,
    is_filer_a_new_registrant: Option<bool>,
    is_filer_a_well_known_seasoned_issuer: Option<bool>,
    filed_pursuant_to_general_instruction_a2: Option<bool>,
    is_fund_24f2_eligible: Option<bool>,
    action_date: Option<NaiveDate>,
    received_date: Option<NaiveDate>,
    ma_i_individual: Option<String>,
    abs_rule: Option<String>,
    period_start: Option<NaiveDate>,
    no_quarterly_activity: Option<bool>,
    no_annual_activity: Option<bool>,
    abs_asset_class: Option<String>,
    depositor_cik: Option<String>,
    sponsor_cik: Option<String>,
    confirming_copy: Option<Box<Submission>>,
    category: Option<String>,
    registered_entity: Option<bool>,
    depositor: Option<Filer>,
    securitizer: Option<Filer>,
    references_429: Option<String>,
}

impl Submission {
    pub fn from_parts(parts: &Vec<DocumentTree>) -> Result<Self> {
        let mut accession_number = None;
        let mut filing_type = None;
        let mut public_document_count: Option<u32> = None;
        let mut items = None;
        let mut filing_date = None;
        let mut date_of_filing_date_change = None;
        let mut effectiveness_date = None;
        let mut filer = None;
        let mut document = None;
        let mut series_and_classes_contracts_data = None;
        let mut period = None;
        let mut reporting_owner = None;
        let mut issuer = None;
        let mut group_members = None;
        let mut subject_company = None;
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
        let mut confirming_copy = None;
        let mut category = None;
        let mut registered_entity = None;
        let mut depositor = None;
        let mut securitizer = None;
        let mut references_429 = None;

        for part in parts {
            match &part {
                DocumentTree::ValueNode(tag, value) => match tag {
                    ValueTag::AccessionNumber => {
                        accession_number = Some(value.clone());
                    }
                    ValueTag::Type => {
                        filing_type = Some(value.clone());
                    }
                    ValueTag::PublicDocumentCount => {
                        public_document_count = Some(value.parse().unwrap());
                    }
                    ValueTag::Items => {
                        items = Some(value.clone());
                    }
                    ValueTag::FilingDate => {
                        filing_date = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::DateOfFilingDateChange => {
                        date_of_filing_date_change =
                            Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::EffectivenessDate => {
                        effectiveness_date = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::Period => {
                        period = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::GroupMembers => {
                        group_members = Some(value.clone());
                    }
                    ValueTag::Reference462B => {
                        reference_462b = Some(value.clone());
                    }
                    ValueTag::IsFilerANewRegistrant => {
                        is_filer_a_new_registrant = Some(parse_bool(value));
                    }
                    ValueTag::IsFilerAWellKnownSeasonedIssuer => {
                        is_filer_a_well_known_seasoned_issuer = Some(parse_bool(value));
                    }
                    ValueTag::FiledPursuantToGeneralInstructionA2 => {
                        filed_pursuant_to_general_instruction_a2 = Some(parse_bool(value));
                    }
                    ValueTag::IsFund24F2Eligible => {
                        is_fund_24f2_eligible = Some(parse_bool(value));
                    }
                    ValueTag::ActionDate => {
                        action_date = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::ReceivedDate => {
                        received_date = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::MaIIndividual => {
                        ma_i_individual = Some(value.clone());
                    }
                    ValueTag::AbsRule => {
                        abs_rule = Some(value.clone());
                    }
                    ValueTag::PeriodStart => {
                        period_start = Some(NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap());
                    }
                    ValueTag::NoQuarterlyActivity => {
                        no_quarterly_activity = Some(parse_bool(value));
                    }
                    ValueTag::NoAnnualActivity => {
                        no_annual_activity = Some(parse_bool(value));
                    }
                    ValueTag::AbsAssetClass => {
                        abs_asset_class = Some(value.clone());
                    }
                    ValueTag::DepositorCik => {
                        depositor_cik = Some(value.clone());
                    }
                    ValueTag::SponsorCik => {
                        sponsor_cik = Some(value.clone());
                    }
                    ValueTag::Category => {
                        category = Some(value.clone())
                    }
                    ValueTag::RegisteredEntity => {
                        registered_entity = Some(parse_bool(value));
                    }
                    ValueTag::References429 => {
                        references_429 = Some(value.clone());
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::Paper => {
                        // TODO: is this right?
                        return Submission::from_parts(parts);
                    }
                    ContainerTag::Filer => {
                        filer = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::Document => {
                        document = Some(Document::from_parts(parts)?);
                    }
                    ContainerTag::SeriesAndClassesContractsData => {
                        series_and_classes_contracts_data = Some(SeriesAndClassesContractsData::from_parts(parts)?);
                    }
                    ContainerTag::ReportingOwner => {
                        reporting_owner = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::Issuer => {
                        issuer = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::SubjectCompany => {
                        subject_company = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::FiledBy => {
                        filed_by = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::ConfirmingCopy => {
                        confirming_copy = Some(Box::new(Submission::from_parts(parts)?));
                    }
                    ContainerTag::Depositor => {
                        depositor = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::Securitizer => {
                        securitizer = Some(Filer::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        Ok(Submission {
            accession_number,
            filing_type,
            items,
            filing_date,
            date_of_filing_date_change,
            effectiveness_date,
            filer,
            document,
            series_and_classes_contracts_data,
            period,
            reporting_owner,
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
            confirming_copy,
            category,
            registered_entity,
            depositor,
            securitizer,
            references_429,
        })
    }
}
