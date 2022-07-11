import React from 'react';
import PropTypes from 'prop-types';
import styles from './ControlPanel.module.css';

const ControlPanel = (props) => {
  const onAmountOfDataChange = (event) => {
    let value = event.target.value;
    if (value === '') {
      value = 0;
    }
    value = parseInt(value, 10);
    if (isNaN(value)) {
      console.log(`Amount of data must be an integer, falling back to ${props.amountOfData}`);
      value = props.amountOfData;
    }
    props.onAmountOfDataChange(value);
  }

  const onSearchTextChange = (event) => {
    props.onSearchTextChange(event.target.value);
  }

  return (
    <div className={styles.ControlPanel} data-testid="ControlPanel">
      <label htmlFor="amountOfData">Amount of test data to generate: </label>
      <input type="text" id="amountOfData" name="amountOfData" value={props.amountOfData} onInput={(event) => onAmountOfDataChange(event)} />
      <button onClick={(event) => props.onGenerateData(props.amountOfData)}>Generate</button>
      <br />
      <label htmlFor="searchText">Search for text: </label>
      <input type="text" id="searchText" name="searchText" value={props.searchText} onInput={(event) => onSearchTextChange(event)} />
    </div>
  );
}

ControlPanel.propTypes = {
  amountOfData: PropTypes.number,
  searchText: PropTypes.string,
  onAmountOfDataChange: PropTypes.func,
  onGenerateData: PropTypes.func,
  onSearchTextChange: PropTypes.func,
};

ControlPanel.defaultProps = {
  amountOfData: 0,
  searchText: '',
  onAmountOfDataChange: () => { },
  onGenerateData: () => { },
  onSearchTextChange: () => { },
};

export default ControlPanel;
