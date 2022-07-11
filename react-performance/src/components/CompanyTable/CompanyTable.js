import React from 'react';
import PropTypes from 'prop-types';

const CompanyTable = (props) => {
  let companiesJSX = props.companies.slice(0, 10).map((company, index) => (
    <tr key={index}>
      <td>{company.name}</td>
      <td>{company.catchPhrase}</td>
      <td>{company.industry}</td>
      <td>{company.phoneNumber}</td>
      <td>{company.email}</td>
      <td>{company.ipv4}</td>
    </tr>
  ))


  return (
    <div>
      <p style={{ margin: '8px', color: 'red' }} >⚠ We only show the first 10 elements of the list to make sure the html dom manipulation doesn't slow down too much ⚠</p>
      <table style={{ display: 'block', height: '400px', width: '70%', overflow: 'auto', marginLeft: 'auto', marginRight: 'auto' }}>
        <thead>
          <tr>
            <th style={{ position: 'sticky', top: 0 }}>Name</th>
            <th style={{ position: 'sticky', top: 0 }}>Catch Phrase</th>
            <th style={{ position: 'sticky', top: 0 }}>Industry</th>
            <th style={{ position: 'sticky', top: 0 }}>Phone Number</th>
            <th style={{ position: 'sticky', top: 0 }}>Email</th>
            <th style={{ position: 'sticky', top: 0 }}>IPv4</th>
          </tr>
        </thead>
        <tbody>
          {companiesJSX}
        </tbody>
      </table>
      <br />
      <p style={{ margin: '4px' }}>Amount of items: {props.companies.length}</p>
    </div>
  );
}

CompanyTable.propTypes = {
  companies: PropTypes.array
};

CompanyTable.defaultProps = {
  companies: []
};

export default CompanyTable;
