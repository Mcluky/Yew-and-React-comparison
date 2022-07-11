use std::rc::Rc;
use yew::{html, Properties};
use yew::prelude::*;

use crate::model::company::Company;

#[derive(Properties, PartialEq)]
pub struct CompanyTableProperties {
    pub companies: Rc<Vec<Rc<Company>>>,
}

#[function_component(CompanyTable)]
pub fn company_table(props: &CompanyTableProperties) -> Html {
    let companies = &props.companies;
    html! {
        <div>
        <p style="margin: 8px; color: red" >{ "⚠ We only show the first 10 elements of the list to make sure the html dom manipulation doesn't slow down too much ⚠" }</p>
        <table style="display: block; height: 400px; width: 70%; overflow: auto; margin-left: auto; margin-right: auto;">
            <thead>
                <tr>
                    <th style="position: sticky; top: 0;">{ "Name" }</th>
                    <th style="position: sticky; top: 0;">{ "Catch Phrase" }</th>
                    <th style="position: sticky; top: 0;">{ "Industry" }</th>
                    <th style="position: sticky; top: 0;">{ "Phone Number" }</th>
                    <th style="position: sticky; top: 0;">{ "Email" }</th>
                    <th style="position: sticky; top: 0;">{ "IPv4" }</th>
                </tr>
            </thead>
            <tbody>
                { for companies.iter().take(10).map(|company| {
                    html! {
                        <tr>
                            <td>{ &company.name }</td>
                            <td>{ &company.catch_phrase }</td>
                            <td>{ &company.industry }</td>
                            <td>{ &company.phone_number }</td>
                            <td>{ &company.email }</td>
                            <td>{ &company.ipv4 }</td>
                        </tr>
                    }
                }) }
            </tbody>
        </table>
        <br/>
        <p style="margin: 4px;" >{ "Amount of items: " }{ companies.len() }</p>
        </div>
    }
}