use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Request {
    version: &'static str,
    queries: Vec<RequestQueryWrapper>,
    cancel_queries: Vec<()>,
    model_id: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct RequestQueryWrapper {
    query: RequestQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_options: Option<usize>,
    query_id: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_context: Option<ApplicationContext>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct ApplicationContext {
    dataset_id: String,
    sources: Vec<Source>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Source {
    report_id: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct RequestQuery {
    commands: Vec<RequestCommand>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct RequestCommand {
    semantic_query_data_shape_command: QueryWrapper,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct QueryWrapper {
    query: Query,
    binding: Binding,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Query {
    version: usize,
    from: Vec<FromClause>,
    select: Vec<SelectClause>,
    #[serde(rename="Where", skip_serializing_if = "Option::is_none")]
    where_clause: Option<Vec<ConditionWrapper>>,
    order_by: Vec<OrderByClause>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct ConditionWrapper {
    condition: Condition,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
enum Condition {
    #[serde(rename_all="PascalCase")]
    In {
        expressions: Vec<OrderByExpression>,
        values: Vec<Vec<LiteralWrapper>>,
    },
    #[serde(rename_all="PascalCase")]
    Not {
        expression: ComparisonWrapper,
    },
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct LiteralWrapper {
    literal: Literal,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Literal {
    value: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct ComparisonWrapper {
    comparison: Comparison,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Comparison {
    comparison_kind: usize,
    left: OrderByExpression,
    right: LiteralWrapper,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct FromClause {
    name: String,
    entity: String,
    #[serde(rename="Type")]
    from_type: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct SelectClause {
    column: SelectColumn,
    name: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct SelectColumn {
    expression: SelectExpression,
    property: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct SelectExpression {
    source_ref: SourceRef,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct SourceRef {
    source: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct OrderByClause {
    direction: usize,
    expression: OrderByExpression,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct OrderByExpression {
    column: SelectColumn,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Binding {
    primary: Primary,
    data_reduction: DataReduction,
    version: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Primary {
    groupings: Vec<Grouping>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct Grouping {
    projections: Vec<usize>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct DataReduction {
    data_volume: usize,
    primary: DataReductionPrimary,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct DataReductionPrimary {
    window: DataReductionWindow,
}

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
struct DataReductionWindow {
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_tokens: Option<Vec<Vec<String>>>,
}

pub fn get_initial() -> Request {
    get_index(None)
}

pub fn get_index(restart_tokens: Option<Vec<String>>) -> Request {
    Request {
        version: "1.0.0",
        queries: vec![
            RequestQueryWrapper {
                query: RequestQuery {
                    commands: vec![
                        RequestCommand {
                            semantic_query_data_shape_command: QueryWrapper {
                                query: Query {
                                    version: 2,
                                    from: vec![
                                        FromClause {
                                            name: "q1".into(),
                                            entity: "CCRB Active - Oracle".into(),
                                            from_type: 0,
                                        },
                                    ],
                                    select: vec![
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Unique Id".into(),
                                            },
                                            name: "Query1.Unique Id".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Command".into(),
                                            },
                                            name: "Query1.Command1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Last Name".into(),
                                            },
                                            name: "Query1.Last Name1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "First Name".into(),
                                            },
                                            name: "Query1.First Name1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Rank".into(),
                                            },
                                            name: "Query1.Rank1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Shield No".into(),
                                            },
                                            name: "Query1.ShieldNo".into(),
                                        },
                                    ],
                                    where_clause: None,
                                    order_by: vec![
                                        OrderByClause {
                                            direction: 1,
                                            expression: OrderByExpression {
                                                column: SelectColumn {
                                                    expression: SelectExpression {
                                                        source_ref: SourceRef {
                                                            source: "q1".into(),
                                                        },
                                                    },
                                                    property: "Command".into(),
                                                },
                                            },
                                        },
                                    ],
                                },
                                binding: Binding {
                                    primary: Primary {
                                        groupings: vec![
                                            Grouping {
                                                projections: vec![0, 1, 2, 3, 4, 5],
                                            }
                                        ],
                                    },
                                    data_reduction: DataReduction {
                                        data_volume: 3,
                                        primary: DataReductionPrimary {
                                            window: DataReductionWindow {
                                                count: 500,
                                                restart_tokens: restart_tokens.map(|v| vec![v]),
                                            },
                                        },
                                    },
                                    version: 1,
                                },
                            },
                        },
                    ],
                },
                cache_options: Some(7),
                query_id: "",
                application_context: Some(ApplicationContext {
                    dataset_id: "523ab509-8e2d-43ed-bfad-11fcd05180d7".into(),
                    sources: vec![
                        Source {
                            report_id: "f508555a-b39d-4c10-8d46-a14bc282e079".into(),
                        }
                    ],
                }),
            }
        ],
        cancel_queries: vec![],
        model_id: 404287,
    }
}

pub fn get_request() -> Request {
    Request {
        version: "1.0.0",
        queries: vec![
            RequestQueryWrapper {
                query: RequestQuery {
                    commands: vec![
                        RequestCommand {
                            semantic_query_data_shape_command: QueryWrapper {
                                query: Query {
                                    version: 2,
                                    from: vec![
                                        FromClause {
                                            name: "q1".into(),
                                            entity: "CCRB Active - Oracle".into(),
                                            from_type: 0,
                                        },
                                    ],
                                    select: vec![
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Unique Id".into(),
                                            },
                                            name: "Query1.Unique Id".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Command".into(),
                                            },
                                            name: "Query1.Command1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Last Name".into(),
                                            },
                                            name: "Query1.Last Name1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "First Name".into(),
                                            },
                                            name: "Query1.First Name1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Rank".into(),
                                            },
                                            name: "Query1.Rank1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Shield No".into(),
                                            },
                                            name: "Query1.ShieldNo".into(),
                                        },
                                    ],
                                    where_clause: None,
                                    order_by: vec![
                                        OrderByClause {
                                            direction: 1,
                                            expression: OrderByExpression {
                                                column: SelectColumn {
                                                    expression: SelectExpression {
                                                        source_ref: SourceRef {
                                                            source: "q1".into(),
                                                        },
                                                    },
                                                    property: "Command".into(),
                                                },
                                            },
                                        },
                                    ],
                                },
                                binding: Binding {
                                    primary: Primary {
                                        groupings: vec![
                                            Grouping {
                                                projections: vec![0, 1, 2, 3, 4, 5],
                                            }
                                        ],
                                    },
                                    data_reduction: DataReduction {
                                        data_volume: 3,
                                        primary: DataReductionPrimary {
                                            window: DataReductionWindow {
                                                count: 500,
                                                restart_tokens: Some(vec![vec![
                                                    "'007 DET'".into(),
                                                    "'001133'".into(),
                                                    "'Isolano'".into(),
                                                    "'Nicholas'".into(),
                                                    "'Detective'".into(),
                                                    "'00545'".into(),
                                                ]]),
                                            },
                                        },
                                    },
                                    version: 1,
                                },
                            },
                        },
                    ],
                },
                cache_options: None,
                query_id: "",
                application_context: None,
            },
        ],
        cancel_queries: vec![],
        model_id: 404287,
    }
}


pub fn get_followup() -> Request {
    Request {
        version: "1.0.0",
        queries: vec![
            RequestQueryWrapper {
                query: RequestQuery {
                    commands: vec![
                        RequestCommand {
                            semantic_query_data_shape_command: QueryWrapper {
                                query: Query {
                                    version: 2,
                                    from: vec![
                                        FromClause {
                                            name: "q1".into(),
                                            entity: "CCRB Active - Oracle".into(),
                                            from_type: 0,
                                        },
                                    ],
                                    select: vec![
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Rn".into(),
                                            },
                                            name: "Sum(Query1.Rn)".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Complaint ID".into(),
                                            },
                                            name: "CountNonNull(Query1.Complaint Id)1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Incident Date".into(),
                                            },
                                            name: "Query1.Incident Date".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "FADO Type".into(),
                                            },
                                            name: "Query1.FADO Type1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Allegation".into(),
                                            },
                                            name: "Query1.Allegation1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Board Disposition".into(),
                                            },
                                            name: "Query1.Board Disposition1".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "NYPD Disposition".into(),
                                            },
                                            name: "Query1.NYPD Disposition".into(),
                                        },
                                        SelectClause {
                                            column: SelectColumn {
                                                expression: SelectExpression {
                                                    source_ref: SourceRef {
                                                        source: "q1".into(),
                                                    },
                                                },
                                                property: "Penalty".into(),
                                            },
                                            name: "Query1.PenaltyDesc1".into(),
                                        },
                                    ],
                                    where_clause: Some(vec![
                                        ConditionWrapper {
                                            condition: Condition::Not {
                                                expression: ComparisonWrapper {
                                                    comparison: Comparison {
                                                        comparison_kind: 0,
                                                        left: OrderByExpression {
                                                            column: SelectColumn {
                                                                expression: SelectExpression {
                                                                    source_ref: SourceRef {
                                                                        source: "q1".into(),
                                                                    },
                                                                },
                                                                property: "Rn".into(),
                                                            },
                                                        },
                                                        right: LiteralWrapper {
                                                            literal: Literal {
                                                                value: "0L".into(),
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                        ConditionWrapper {
                                            condition: Condition::In {
                                                expressions: vec![
                                                    OrderByExpression {
                                                        column: SelectColumn {
                                                            expression: SelectExpression {
                                                                source_ref: SourceRef {
                                                                    source: "q1".into(),
                                                                },
                                                            },
                                                            property: "Shield No".into(),
                                                        },
                                                    },
                                                ],
                                                values: vec![vec![
                                                    LiteralWrapper {
                                                        literal: Literal {
                                                            value: "\'03057\'".into(),
                                                        },
                                                    },
                                                ]],
                                            }
                                        },
                                        ConditionWrapper {
                                            condition: Condition::In {
                                                expressions: vec![
                                                    OrderByExpression {
                                                        column: SelectColumn {
                                                            expression: SelectExpression {
                                                                source_ref: SourceRef {
                                                                    source: "q1".into(),
                                                                },
                                                            },
                                                            property: "Last Name".into(),
                                                        },
                                                    },
                                                ],
                                                values: vec![vec![
                                                    LiteralWrapper {
                                                        literal: Literal {
                                                            value: "\'Eysel\'".into(),
                                                        },
                                                    },
                                                ]],
                                            }
                                        },
                                        ConditionWrapper {
                                            condition: Condition::In {
                                                expressions: vec![
                                                    OrderByExpression {
                                                        column: SelectColumn {
                                                            expression: SelectExpression {
                                                                source_ref: SourceRef {
                                                                    source: "q1".into(),
                                                                },
                                                            },
                                                            property: "First Name".into(),
                                                        },
                                                    },
                                                ],
                                                values: vec![vec![
                                                    LiteralWrapper {
                                                        literal: Literal {
                                                            value: "\'Robert\'".into(),
                                                        },
                                                    },
                                                ]],
                                            }
                                        },
                                        ConditionWrapper {
                                            condition: Condition::In {
                                                expressions: vec![
                                                    OrderByExpression {
                                                        column: SelectColumn {
                                                            expression: SelectExpression {
                                                                source_ref: SourceRef {
                                                                    source: "q1".into(),
                                                                },
                                                            },
                                                            property: "Unique Id".into(),
                                                        },
                                                    },
                                                ],
                                                values: vec![vec![
                                                    LiteralWrapper {
                                                        literal: Literal {
                                                            value: "\'000019\'".into(),
                                                        },
                                                    },
                                                ]],
                                            }
                                        },
                                        ConditionWrapper {
                                            condition: Condition::In {
                                                expressions: vec![
                                                    OrderByExpression {
                                                        column: SelectColumn {
                                                            expression: SelectExpression {
                                                                source_ref: SourceRef {
                                                                    source: "q1".into(),
                                                                },
                                                            },
                                                            property: "Command".into(),
                                                        },
                                                    },
                                                ],
                                                values: vec![vec![
                                                    LiteralWrapper {
                                                        literal: Literal {
                                                            value: "\'001 DET\'".into(),
                                                        },
                                                    },
                                                ]],
                                            }
                                        },
                                        ConditionWrapper {
                                            condition: Condition::In {
                                                expressions: vec![
                                                    OrderByExpression {
                                                        column: SelectColumn {
                                                            expression: SelectExpression {
                                                                source_ref: SourceRef {
                                                                    source: "q1".into(),
                                                                },
                                                            },
                                                            property: "Rank".into(),
                                                        },
                                                    },
                                                ],
                                                values: vec![vec![
                                                    LiteralWrapper {
                                                        literal: Literal {
                                                            value: "\'Police Officer\'".into(),
                                                        },
                                                    },
                                                ]],
                                            }
                                        },
                                    ]),
                                    order_by: vec![
                                        OrderByClause {
                                            direction: 1,
                                            expression: OrderByExpression {
                                                column: SelectColumn {
                                                    expression: SelectExpression {
                                                        source_ref: SourceRef {
                                                            source: "q1".into(),
                                                        },
                                                    },
                                                    property: "Rn".into(),
                                                },
                                            },
                                        },
                                    ],
                                },
                                binding: Binding {
                                    primary: Primary {
                                        groupings: vec![
                                            Grouping {
                                                projections: vec![0, 1, 2, 3, 4, 5, 6, 7],
                                            }
                                        ],
                                    },
                                    data_reduction: DataReduction {
                                        data_volume: 3,
                                        primary: DataReductionPrimary {
                                            window: DataReductionWindow {
                                                count: 500,
                                                restart_tokens: None,
                                            },
                                        },
                                    },
                                    version: 1,
                                },
                            },
                        },
                    ],
                },
                cache_options: None,
                query_id: "",
                application_context: Some(ApplicationContext {
                    dataset_id: "523ab509-8e2d-43ed-bfad-11fcd05180d7".into(),
                    sources: vec![
                        Source {
                            report_id: "f508555a-b39d-4c10-8d46-a14bc282e079".into(),
                        }
                    ],
                }),
            }
        ],
        cancel_queries: vec![],
        model_id: 404287,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_query() {
        let req = get_request();

        const EXPECTED: &str = "{\"version\":\"1.0.0\",\"queries\":[{\"Query\":{\"Commands\":[{\"SemanticQueryDataShapeCommand\":{\"Query\":{\"Version\":2,\"From\":[{\"Name\":\"q1\",\"Entity\":\"CCRB Active - Oracle\",\"Type\":0}],\"Select\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Unique Id\"},\"Name\":\"Query1.Unique Id\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Command\"},\"Name\":\"Query1.Command1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Last Name\"},\"Name\":\"Query1.Last Name1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"First Name\"},\"Name\":\"Query1.First Name1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Rank\"},\"Name\":\"Query1.Rank1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Shield No\"},\"Name\":\"Query1.ShieldNo\"}],\"OrderBy\":[{\"Direction\":1,\"Expression\":{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Command\"}}}]},\"Binding\":{\"Primary\":{\"Groupings\":[{\"Projections\":[0,1,2,3,4,5]}]},\"DataReduction\":{\"DataVolume\":3,\"Primary\":{\"Window\":{\"Count\":500,\"RestartTokens\":[[\"\'007 DET\'\",\"\'001133\'\",\"\'Isolano\'\",\"\'Nicholas\'\",\"\'Detective\'\",\"\'00545\'\"]]}}},\"Version\":1}}}]},\"QueryId\":\"\"}],\"cancelQueries\":[],\"modelId\":404287}";

        let actual = serde_json::to_string(&req).expect("serialize");

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn serialize_followup() {
        let req = get_followup();

        const EXPECTED: &str = "{\"version\":\"1.0.0\",\"queries\":[{\"Query\":{\"Commands\":[{\"SemanticQueryDataShapeCommand\":{\"Query\":{\"Version\":2,\"From\":[{\"Name\":\"q1\",\"Entity\":\"CCRB Active - Oracle\",\"Type\":0}],\"Select\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Rn\"},\"Name\":\"Sum(Query1.Rn)\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Complaint ID\"},\"Name\":\"CountNonNull(Query1.Complaint Id)1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Incident Date\"},\"Name\":\"Query1.Incident Date\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"FADO Type\"},\"Name\":\"Query1.FADO Type1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Allegation\"},\"Name\":\"Query1.Allegation1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Board Disposition\"},\"Name\":\"Query1.Board Disposition1\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"NYPD Disposition\"},\"Name\":\"Query1.NYPD Disposition\"},{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Penalty\"},\"Name\":\"Query1.PenaltyDesc1\"}],\"Where\":[{\"Condition\":{\"Not\":{\"Expression\":{\"Comparison\":{\"ComparisonKind\":0,\"Left\":{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Rn\"}},\"Right\":{\"Literal\":{\"Value\":\"0L\"}}}}}}},{\"Condition\":{\"In\":{\"Expressions\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Shield No\"}}],\"Values\":[[{\"Literal\":{\"Value\":\"\'03057\'\"}}]]}}},{\"Condition\":{\"In\":{\"Expressions\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Last Name\"}}],\"Values\":[[{\"Literal\":{\"Value\":\"\'Eysel\'\"}}]]}}},{\"Condition\":{\"In\":{\"Expressions\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"First Name\"}}],\"Values\":[[{\"Literal\":{\"Value\":\"\'Robert\'\"}}]]}}},{\"Condition\":{\"In\":{\"Expressions\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Unique Id\"}}],\"Values\":[[{\"Literal\":{\"Value\":\"\'000019\'\"}}]]}}},{\"Condition\":{\"In\":{\"Expressions\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Command\"}}],\"Values\":[[{\"Literal\":{\"Value\":\"\'001 DET\'\"}}]]}}},{\"Condition\":{\"In\":{\"Expressions\":[{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Rank\"}}],\"Values\":[[{\"Literal\":{\"Value\":\"\'Police Officer\'\"}}]]}}}],\"OrderBy\":[{\"Direction\":1,\"Expression\":{\"Column\":{\"Expression\":{\"SourceRef\":{\"Source\":\"q1\"}},\"Property\":\"Rn\"}}}]},\"Binding\":{\"Primary\":{\"Groupings\":[{\"Projections\":[0,1,2,3,4,5,6,7]}]},\"DataReduction\":{\"DataVolume\":3,\"Primary\":{\"Window\":{\"Count\":500}}},\"Version\":1}}}]},\"QueryId\":\"\",\"ApplicationContext\":{\"DatasetId\":\"523ab509-8e2d-43ed-bfad-11fcd05180d7\",\"Sources\":[{\"ReportId\":\"f508555a-b39d-4c10-8d46-a14bc282e079\"}]}}],\"cancelQueries\":[],\"modelId\":404287}";

        let actual = serde_json::to_string(&req).expect("serialize");

        assert_eq!(actual, EXPECTED);
    }
}
