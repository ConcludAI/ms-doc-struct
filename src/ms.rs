use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub status: String,
    #[serde(rename = "createdDateTime")]
    pub created: String,
    #[serde(rename = "lastUpdatedDateTime")]
    pub updated: String,
    #[serde(rename = "analyzeResult")]
    pub analysis: Analysis
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Analysis {
    pub content: String,
    pub pages: Vec<Page>,
    pub tables: Vec<Table>,
    pub paragraphs: Vec<Para>,
    pub styles: Vec<Style>
}

// modelled after https://learn.microsoft.com/en-gb/azure/applied-ai-services/form-recognizer/concept-layout?view=form-recog-3.0.0
// "pages": [
//     {
//         "pageNumber": 1,
//         "angle": 0,
//         "width": 915,
//         "height": 1190,
//         "unit": "pixel",
//         "words": [],
//         "lines": [],
//         "spans": [],
//         "kind": "document"
//     }
// ]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "pageNumber")]
    pub page_num: usize,
    pub width: f32,
    pub height: f32,
    pub unit: String,
    pub words: Vec<Word>,
    pub lines: Vec<Line>,
    pub spans: Vec<Span>,
    pub kind: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Word {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Span {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Table {}

// modelled after https://learn.microsoft.com/en-gb/azure/applied-ai-services/form-recognizer/concept-layout?view=form-recog-3.0.0
// {
//     "paragraphs": [
//             {
//                 "spans": [],
//                 "boundingRegions": [],
//                 "role": "title",
//                 "content": "NEWS TODAY"
//             },
//             {
//                 "spans": [],
//                 "boundingRegions": [],
//                 "role": "sectionHeading",
//                 "content": "Mirjam Nilsson"
//             }
//     ]
// }
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Para {
    pub spans: Vec<Span>,
    #[serde(rename = "boundingRegions")]
    pub regions: Vec<BoundingRegion>,
    pub role: Option<ParaRole>,
    pub content: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParaRole {
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "sectionHeading")]
    SectionHeading,
    #[serde(rename = "footnote")]
    Footnote,
    #[serde(rename = "pageHeader")]
    PageHeader,
    #[serde(rename = "pageFooter")]
    PageFooter,
    #[serde(rename = "pageNumber")]
    PageNumber
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BoundingRegion {
    #[serde(rename = "pageNumber")]
    pub page_num: usize,
    pub polygon: Polygon

}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Style {}

pub type Polygon = Vec<f32>;