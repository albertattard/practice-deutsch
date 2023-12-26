# Practice Deutsch

A simple command line application to practice German.  This application supports the following practices

1. Practice German articles (_der_, _die_, and _das_)

   ```shell
   $ practice-deutsch articles
   ```

   Listen to the noun and then type the article

2. Practice German numbers

   ```shell
   $ practice-deutsch numbers
   ```

   Listen to the number and then type the number

3. Practice German alphabet

   ```shell
   $ practice-deutsch alphabet
   ```

   Listen to the letter of the alphabet and then type it

## Useful Commands

1. Build the project

   ```shell
   $ cargo build --release
   ```

   This will create a binary file `target/release/practice-deutsch`.

2. Run the project

   ```shell
   $ ./target/release/practice-deutsch --help
   ```
