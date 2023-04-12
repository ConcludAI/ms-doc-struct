use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutForStatus {
    pub status: String,
}

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
    pub kind: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Word {}

// modelled after
// {
//     "content": "Some Content",
//     "polygon": [
//         557,
//         84,
//         1111,
//         85,
//         1111,
//         156,
//         557,
//         154
//     ],
//     "spans": [
//         {
//             "offset": 567,
//             "length": 13
//         }
//     ]
// }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Span {}

// modelled after: https://westus.dev.cognitive.microsoft.com/docs/services/form-recognizer-api-2022-08-31/operations/GetAnalyzeDocumentResult
// "tables": [
//     {
//         "rowCount": 1,                    // Number of rows in table
//         "columnCount": 1,                 // Number of columns in table
//         "boundingRegions": [              // Bounding boxes potentially across pages covered by table
//         {
//             "pageNumber": 1,              // 1-indexed page number
//             "boundingBox": [ ... ],       // Bounding box
//         }
//         ],
//         "spans": [ ... ],                 // Parts of top-level content covered by table
//         // List of cells in table
//         "cells": [
//         {
//             "kind": "stubHead",           // Cell kind: content (default), rowHeader, columnHeader, stubHead, description
//             "rowIndex": 0,                // 0-indexed row position of cell
//             "columnIndex": 0,             // 0-indexed column position of cell
//             "rowSpan": 1,                 // Number of rows spanned by cell (default=1)
//             "columnSpan": 1,              // Number of columns spanned by cell (default=1)
//             "content": "SALESPERSON",     // Concatenated content of cell
//             "boundingRegions": [ ... ],   // Bounding regions covered by cell
//             "spans": [ ... ]              // Parts of top-level content covered by cell
//         }, ...
//         ]
//     }, ...
// ]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "rowCount")]
    pub num_row: usize,
    #[serde(rename = "columnCount")]
    pub num_col: usize,
    #[serde(rename = "boundingRegions")]
    pub regions: Vec<BoundingRegion>,
    pub cells: Vec<Cell>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cell {
    kind: Option<CellKind>,
    #[serde(rename = "rowIndex")]
    row_idx: usize,
    #[serde(rename = "columnIndex")]
    col_idx: usize,
    #[serde(rename = "rowSpan")]
    row_span: Option<usize>,
    #[serde(rename = "columnSpan")]
    col_span: Option<usize>,
    content: String,
    #[serde(rename = "boundingRegions")]
    pub regions: Vec<BoundingRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CellKind {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "rowHeader")]
    RowHead,
    #[serde(rename = "columnHeader")]
    ColHead,
    #[serde(rename = "stubHead")]
    StubHead,
    #[serde(rename = "description")]
    Description
}

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