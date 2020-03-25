# Proof of concept for python+js+rust marriage

## requirements

`nightly` version of rust compiler
```bash
rustup install nightly
```

### python requirements
python3.5+, pip, virtualenv
https://github.com/PyO3/maturin
https://github.com/PyO3/PyO3

### js requirements
https://rustwasm.github.io/docs/book/game-of-life/setup.html


## building
```bash
./build.sh js  # or python
```

## running
### python
```bash
./build.sh python
cd ./example_python
./main.py
```

### javascript
```bash
cd example-javascript/www
npm install
npm run start
# visit localhost:8080 for result
```