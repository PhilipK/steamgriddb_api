use crate::{
    dimensions::{GridDimentions, HeroDimentions},
    queries::{parameters_to_qeury, to_qeury_string, to_qeury_string_single, ToQuerys},
    shared_settings::{AnimtionType, Humor, MimeType, MimeTypeLogo, Nsfw},
    styles::Style,
};

pub enum QueryType<'a> {
    Grid(Option<GridQueryParameters<'a>>),
    Hero(Option<HeroQueryParameters<'a>>),
    Logo(Option<LogoQueryParameters<'a>>),
    Icon(Option<IconQueryParameters<'a>>),
}

impl ToQuerys for QueryType<'_> {
    fn to_querys(&self) -> String {
        match self {
            QueryType::Grid(None)
            | QueryType::Hero(None)
            | QueryType::Logo(None)
            | QueryType::Icon(None) => "".to_string(),
            QueryType::Grid(Some(grid_query_parameters)) => grid_query_parameters.to_querys(),
            QueryType::Hero(Some(hero_query_parameters)) => hero_query_parameters.to_querys(),
            QueryType::Logo(Some(logo_query_parameters)) => logo_query_parameters.to_querys(),
            QueryType::Icon(Some(icon_query_parameters)) => icon_query_parameters.to_querys(),
        }
    }
}

#[derive(Default)]
pub struct HeroQueryParameters<'a> {
    pub styles: Option<&'a [Style]>,
    pub dimentions: Option<&'a [HeroDimentions]>,
    pub mimes: Option<&'a [MimeType]>,
    pub types: Option<&'a [AnimtionType]>,
    pub nsfw: Option<&'a Nsfw>,
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for HeroQueryParameters<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.dimentions),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}

#[derive(Default)]
pub struct GridQueryParameters<'a> {
    pub styles: Option<&'a [Style]>,
    pub dimentions: Option<&'a [GridDimentions]>,
    pub mimes: Option<&'a [MimeType]>,
    pub types: Option<&'a [AnimtionType]>,
    pub nsfw: Option<&'a Nsfw>,
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for GridQueryParameters<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.dimentions),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}

#[derive(Default)]
pub struct LogoQueryParameters<'a> {
    pub styles: Option<&'a [Style]>,
    pub mimes: Option<&'a [MimeTypeLogo]>,
    pub types: Option<&'a [AnimtionType]>,
    pub nsfw: Option<&'a Nsfw>,
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for LogoQueryParameters<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}

#[derive(Default)]
pub struct IconQueryParameters<'a> {
    pub styles: Option<&'a [Style]>,
    pub mimes: Option<&'a [MimeTypeLogo]>,
    pub types: Option<&'a [AnimtionType]>,
    pub nsfw: Option<&'a Nsfw>,
    pub humor: Option<&'a Humor>,
}

impl ToQuerys for IconQueryParameters<'_> {
    fn to_querys(&self) -> String {
        parameters_to_qeury(&[
            to_qeury_string(self.styles),
            to_qeury_string(self.mimes),
            to_qeury_string(self.types),
            to_qeury_string_single(self.nsfw),
            to_qeury_string_single(self.humor),
        ])
    }
}
