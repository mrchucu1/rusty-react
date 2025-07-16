import './style.css';
import init, { render } from 'rusty-react';

async function run() {
  await init();

  render('root');
}

run();

console.log('Rust-Wasm module loaded and render function called.');
