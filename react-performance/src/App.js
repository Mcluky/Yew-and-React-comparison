import logo from './logo.svg';
import './App.css';
import ControlPanel from './components/ControlPanel/ControlPanel';
import React from 'react';
import CompanyTable from './components/CompanyTable/CompanyTable';
import Company from './model/company';

function App() {
  const [amountOfData, setAmountOfData] = React.useState(0);
  const [searchText, setSearchText] = React.useState('');
  const [companies, setCompanies] = React.useState([]);
  const [filteredCompanies, setFilteredCompanies] = React.useState([]);
  const [lastRequiredTimeForSearch, setLastRequiredTimeForSearch] = React.useState(0);

  const onGenerateData = (amountOfData) => {
    const newCompanies = [];
    for (let i = 0; i < amountOfData; i++) {
      newCompanies.push(Company.generateFake());
    }
    setCompanies(newCompanies);
    setFilteredCompanies(newCompanies);
    setSearchText('');
  }

  const onSearchTextChange = (searchText) => {
    setSearchText(searchText);
    let searchTextCaseInsensitive = searchText.toLowerCase();
    // start stop time here 
    const startTime = new Date().getTime();
    const newFilteredCompanies = companies.filter(company => company.contains(searchTextCaseInsensitive));
    // stop time here
    const endTime = new Date().getTime();
    console.log(`Searching for ${searchText} took ${endTime - startTime} ms`);
    setLastRequiredTimeForSearch(endTime - startTime);
    setFilteredCompanies(newFilteredCompanies);
  }

  return (
    <main>
      <h1>{"Hello, this is a test of react, the javascript framework ⚛️!"}</h1>
      <ControlPanel amountOfData={amountOfData} onAmountOfDataChange={setAmountOfData} onGenerateData={onGenerateData} searchText={searchText} onSearchTextChange={onSearchTextChange} />
      <p>{ `Required time for last search: ${lastRequiredTimeForSearch} ms` }</p>
      <CompanyTable companies={filteredCompanies} />
    </main>
  );
}

export default App;
