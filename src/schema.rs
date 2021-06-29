use crate::document_tree::DocumentTree;
use crate::document_tree::DocumentTree::ContainerNode;
use crate::error::Result;
use crate::tag::{ContainerTag, ValueTag};
use crate::types::{parse_bool, MonthDayPair, parse_date};
use chrono::NaiveDate;


#[derive(Debug, PartialEq, Clone)]
pub struct FilingValues {
    form_type: String,
    act: Option<String>,
    file_number: Option<String>,
    film_number: Option<String>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct CompanyData {
    conformed_name: String,
    cik: String,
    irs_number: Option<String>,
    state_of_incorporation: Option<String>,
    fiscal_year_end: Option<MonthDayPair>,
    assigned_sic: Option<String>,
}

impl CompanyData {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
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
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    street1: Option<String>,
    street2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    phone: Option<String>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct FormerCompany {
    former_conformed_name: String,
    date_changed: NaiveDate,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Filer {
    company_data: Option<CompanyData>,
    filing_values: Option<FilingValues>,
    business_address: Option<Address>,
    mail_address: Option<Address>,
    owner_data: Option<CompanyData>,
    former_name: Vec<FormerCompany>,
    former_company: Vec<FormerCompany>,
}

impl Filer {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut company_data = None;
        let mut filing_values = None;
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
                        assert!(filing_values.is_none());
                        filing_values = Some(FilingValues::from_parts(parts)?)
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

        Ok(Filer {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    doc_type: String,
    sequence: u32,
    filename: String,
    text: String, // TODO: parse
    description: Option<String>,
}

impl Document {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut doc_type = None;
        let mut sequence = None;
        let mut filename = None;
        let mut text = None;
        let mut description = None;

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
                    _ => panic!("Unexpected: {:?}", &part),
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

#[derive(Debug, PartialEq, Clone)]
pub struct ClassContract {
    class_contract_id: String,
    class_contract_name: String,
    class_contract_ticker_symbol: Option<String>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Series {
    owner_cik: Option<String>,
    series_id: String,
    series_name: String,
    class_contracts: Vec<ClassContract>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct SeriesAndCik {
    cik: String,
    series: Series,
}

impl SeriesAndCik {
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

        Ok(SeriesAndCik {
            series: series.unwrap(),
            cik: cik.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Merger {
    acquiring_data: SeriesAndCik,
    target_data: SeriesAndCik,
}

impl Merger {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut acquiring_data = None;
        let mut target_data = None;

        for part in parts {
            match &part {
                ContainerNode(tag, parts) => match tag {
                    ContainerTag::AcquiringData => {
                        assert!(acquiring_data.is_none());
                        acquiring_data = Some(SeriesAndCik::from_parts(parts)?)
                    }
                    ContainerTag::TargetData => {
                        assert!(target_data.is_none());
                        target_data = Some(SeriesAndCik::from_parts(parts)?)
                    }
                    _ => panic!("Unexpected: {:?}", &part),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }
        Ok(Merger {
            acquiring_data: acquiring_data.unwrap(),
            target_data: target_data.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SeriesAndClassesContracts {
    series: Vec<Series>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct MergerSeriesAndClassContracts {
    mergers: Vec<Merger>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct SeriesAndClassesContractsData {
    existing_series_and_classes_contracts: Option<SeriesAndClassesContracts>,
    merger_series_and_classes_contracts: Option<MergerSeriesAndClassContracts>,
}

impl SeriesAndClassesContractsData {
    pub fn from_parts(parts: &[DocumentTree]) -> Result<Self> {
        let mut existing_series_and_classes_contracts = None;
        let mut merger_series_and_classes_contracts = None;

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

#[derive(Debug, PartialEq, Clone)]
pub struct Submission {
    accession_number: Option<String>,
    filing_type: Option<String>,
    items: Vec<String>,
    filing_date: Option<NaiveDate>,
    date_of_filing_date_change: Option<NaiveDate>,
    effectiveness_date: Option<NaiveDate>,
    period: Option<NaiveDate>,
    filers: Vec<Filer>,
    documents: Vec<Document>,
    series_and_classes_contracts_data: Option<SeriesAndClassesContractsData>,
    reporting_owners: Vec<Filer>,
    issuer: Option<Filer>,
    group_members: Vec<String>,
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
                        date_of_filing_date_change =
                            Some(parse_date(value));
                    }
                    ValueTag::EffectivenessDate => {
                        assert!(effectiveness_date.is_none());
                        effectiveness_date =
                            Some(parse_date(value));
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
                        received_date =
                            Some(parse_date(value));
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
                    _ => panic!("Unexpected: {:?}", &part),
                },
                DocumentTree::ContainerNode(tag, parts) => match tag {
                    ContainerTag::Paper => {
                        // TODO: is this right?
                        return Submission::from_parts(parts);
                    }
                    ContainerTag::Filer => {
                        let filer = Filer::from_parts(parts)?;
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
                        let reporting_owner = Filer::from_parts(parts)?;
                        reporting_owners.push(reporting_owner);
                    }
                    ContainerTag::Issuer => {
                        assert!(issuer.is_none());
                        issuer = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::SubjectCompany => {
                        assert!(subject_company.is_none());
                        subject_company = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::FiledBy => {
                        assert!(filed_by.is_none());
                        filed_by = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::ConfirmingCopy => {
                        assert!(confirming_copy.is_none());
                        confirming_copy = Some(Box::new(Submission::from_parts(parts)?));
                    }
                    ContainerTag::Depositor => {
                        assert!(depositor.is_none());
                        depositor = Some(Filer::from_parts(parts)?);
                    }
                    ContainerTag::Securitizer => {
                        assert!(securitizer.is_none());
                        securitizer = Some(Filer::from_parts(parts)?);
                    }
                    _ => unimplemented!("{:?}", tag),
                },
                _ => panic!("Unexpected: {:?}", &part),
            }
        }

        assert_eq!(public_document_count, documents.len());

        Ok(Submission {
            accession_number,
            filing_type,
            items,
            filing_date,
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
            confirming_copy,
            category,
            registered_entity,
            depositor,
            securitizer,
            references_429,
            reporting_owners,
        })
    }
}
