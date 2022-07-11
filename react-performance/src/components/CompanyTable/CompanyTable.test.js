import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import CompanyTable from './CompanyTable';

describe('<CompanyTable />', () => {
  test('it should mount', () => {
    render(<CompanyTable />);
    
    const companyTable = screen.getByTestId('CompanyTable');

    expect(companyTable).toBeInTheDocument();
  });
});