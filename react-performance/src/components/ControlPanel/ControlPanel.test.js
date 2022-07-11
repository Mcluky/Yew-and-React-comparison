import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import ControlPanel from './ControlPanel';

describe('<ControlPanel />', () => {
  test('it should mount', () => {
    render(<ControlPanel />);
    
    const controlPanel = screen.getByTestId('ControlPanel');

    expect(controlPanel).toBeInTheDocument();
  });
});