Below is the default task structure representation as **JSON**.
The **Redis** cache *should* use the `tasks` key to store this.

<br />

To store this in **Redis**, copy the **JSON** string below to a file called
`structure.json` and use the following command:
```bash
redis-cli -a <password> SET tasks "$(jq -c . < structure.json)"
```

<br />

```json
{
  "TTD": {
    "Assessing and Managing a Patient in the Outpatient Clinic": [
      "NP Adjuvant",
      "NP Neoadjuvant",
      "NP Metastatic",
      "FU Adjuvant/Neoadjuvant",
      "FU metastatic"
    ]
  },
  "FD": {
    "Providing an Assessment and Basic Management Plan for Patients Seen in Consultation": [
      "RT for DCIS Post BCS",
      "RT for Invasive BC Post BCS Node Negative",
      "RT for Invasive BC Post BCS Node Positive",
      "RT for Invasive BC Post Mastectomy, any Nodal Status",
      "RT of Palliative Indication MBC",
      "HR+/HER2 - Resected Node Negative Premenopausal",
      "HR+/HER2 - Resected Node Negative Postmenopausal",
      "HR+/HER2 - Resected Node Positive Premenopausal",
      "HR+/HER2 - Resected Node Positive Postmenopausal",
      "HR+/HER2 for Neoadjuvant",
      "HR+/HER2 - MBC",
      "HER2+ Resected",
      "HER2+ for Neoadjuvant",
      "HER2+ MBC",
      "TN Resected",
      "TN for Neoadjuvant",
      "TN MBC"
    ],
    "Providing Assessment and Basic Management for Ongoing Care": [
      "RT for Resected DCIS or Invasive BC",
      "RT for Palliative Indication MBC",
      "OncotypeDX Results",
      "Adjuvant/nNeoadjuvant CT +/- H",
      "Adjuvant H (CT Completed, May be Receiving RT +/- HT)",
      "Adjuvant Tam +/- OFS",
      "Adjuvant AI +/- OFS",
      "HR+/HER2- MBC 1L",
      "HR+/HER2- MBC > 1L",
      "HER2+ MBC 1L",
      "HER2+ MBC > 1L",
      "TN MBC 1L",
      "TN MBC > 1L",
      "Supportive Care MBC not on Active Therapy",
      "EBC/MBC on Clinical Trial"
    ],
    "Prescribing Systemic Therapy (Basic Contexts)": [
      "Tamoxifen",
      "AI",
      "OFS",
      "AI + CDK4/6i",
      "Fulvestrant + CDK4/6i",
      "Fulvestrant",
      "DC +/- H",
      "TCH",
      "FEC-D +/- H",
      "DD CT +/-H",
      "Capecitabine",
      "Single Agent Taxane",
      "Eribulin",
      "Pertuzumab Containing Regimen",
      "Trastuzumab Emtansine",
      "Bisphosphonate",
      "Denosumab"
    ],
    "Discussing Serious News": [
      "New Diagnosis MBC",
      "Progression MBC",
      "New complication MBC",
      "GOC",
      "Complication of Therapy"
    ],
    "Assessing and Managing Urgent or Emergent Oncology Scenarios": [
      "Inflammatory Breast Cancer",
      "Visceral Crisis MBC",
      "Brain Met MBC",
      "Spinal Cord Compression MBC",
      "Pain Crisis MBC",
      "DVT/PE in EBC or MBC",
      "Complication of Therapy Requiring Further Investigation/Management"
    ],
    "Coordinating Patient Care to Access Health Services": [
      "Counsel, Coordinate +/- FU Genetic Yesting for Hereditary BC: Mainstreaming or Referral",
      "Referral +/- FU for Common Allied Resources: Psychosocial, DAC, Lymphedema/PT, Rehab Med, Radiology, Pathology, Surgery, RT, Palliative, Transition",
      "Complete Request and Present Patient at Breast, MSK or SBRT Rounds"
    ]
  },
  "CD": {
    "Assessing New Patients Seen in Consultation and Planning Management": [
      "RT for DCIS Post BCS",
      "RT for Invasive BC Post BCS Node Negative",
      "RT for Invasive BC Post BCS Node Positive",
      "RT for Invasive BC Post Mastectomy, any Nodal Status",
      "RT of Palliative Indication MBC",
      "HR+/HER2 - Resected Node Negative Premenopausal",
      "HR+/HER2 - Resected Node Negative Postmenopausal",
      "HR+/HER2 - Resected Node Positive Premenopausal",
      "HR+/HER2 - Resected Node Positive Postmenopausal",
      "HR+/HER2 for Neoadjuvant",
      "HR+/HER2 - MBC",
      "HER2+ Resected",
      "HER2+ for Neoadjuvant",
      "HER2+ MBC",
      "TN Resected",
      "TN for Neoadjuvant",
      "TN MBC"
    ],
    "Providing Comprehensive Assessment and Management for Ongoing Care": [
      "RT for Resected DCIS or Invasive BC",
      "RT for Palliative Indication MBC",
      "OncotypeDX Results",
      "Adjuvant/Neoadjuvant CT +/- H",
      "Adjuvant H (CT Completed, May be Receiving RT +/- HT)",
      "Adjuvant Tam +/- OFS",
      "Adjuvant AI +/- OFS",
      "HR+/HER2- MBC 1L",
      "HR+/HER2- MBC > 1L",
      "HER2+ MBC 1L",
      "HER2+ MBC > 1L",
      "TN MBC 1L",
      "TN MBC > 1L",
      "Supportive Care MBC Not on Active Therapy",
      "EBC/MBC on Clinical Trial"
    ],
    "Prescribing Systemic Therapy": [
      "Tamoxifen",
      "AI",
      "OFS",
      "AI + CDK4/6i",
      "Fulvestrant + CDK4/6i",
      "Fulvestrant",
      "DC +/- H",
      "TCH",
      "FEC-D +/- H",
      "DD CT +/-H",
      "Capecitabine",
      "Single Agent Taxane",
      "Eribulin",
      "Pertuzumab Containing Regimen",
      "Trastuzumab Emtansine",
      "Bisphosphonate",
      "Denosumab"
    ],
    "Transitioning Away from Cancer Center FU +/- Active Anti-Cancer Therapy": [
      "Transition of Care EBC to Family MD",
      "Stopping Active Treatment for MBC"
    ],
    "Providing Longitudinal Outpatient Care to Patients": [
      "Coordinate Urgent Test or Referral",
      "Follow-up and Call Patient with Results of any Pending Tests or Tests Ordered",
      "Follow-up Call Re: Tolerance New Systemic tx or New/Change in Pain Meds",
      "Labs and Orders for Transfusion",
      "Procedural-Related Transition Orders",
      "Blue Cross or Other Special Authorization Form",
      "Palliative Blue Cross Application",
      "Access Drug Available on SAP – Patient Consent, SAP Form, AHS Waiver, DP"
    ],
    "Working with Other Physicians and Healthcare Providers to Provide Multidisciplinary Care": [
      "Counsel, Coordinate +/- FU Genetic Testing for Hereditary BC: Mainstreaming or Referral",
      "Referral +/- FU for Common Allied Resources: Psychosocial, DAC, Lymphedema/PT, Rehab Med, Radiology, Pathology, Surgery, RT, Palliative, Transition",
      "Complete Request and Present Patient at Breast, MSK or SBRT Rounds",
      "Breast Consult on Ward"
    ]
  },
  "TTP": {
    "Managing an Outpatient Practice (With Staff Oversight/Back-up)": [
      "Run NP Breast Clinic and Complete all Resultant Clinic-Related Tasks",
      "Run Breast Treatment Clinic and Complete all Resultant Clinic-Related Tasks",
      "MD Contact for Breast Patients Receiving Chemotherapy in DCU",
      "Disability Insurance Form for Breast Patient Starting Adjuvant Chemotherapy or any MBC Patient",
      "Return to Work Discussion +/- Completion of Form/Letter",
      "Breast Triage",
      "Breast Consults/Specialist Link Contact Person",
      "Chair Breast Rounds"
    ]
  }
}
```