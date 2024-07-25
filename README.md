# chrono db
A Time series Database made for citra open source org 


# Docs

- We are storing all the data in ```.itlg``` custom file format made for IntelliLog Project, Pronounce as "It Log"

### Usage
- ```cargo build``` for building
- ```cargo run <db name> c``` for creating db
- ```cargo run <db name> w "header" "body"``` for writing
- ```cargo run <db name> r``` for reading

### Note
- Data Format ```TIME HEADER BODY```
- The project is still under development, The above are subjected to change

### TO DO
- Encryption & Decryption of file
- Read filters & queries
- Validation for write
- etc for later

### Contributors

**Creator/Founder**: Sugam Kuber ([Github](https://github.com/sugamkuber)) ([LinkedIn](https://linkedin.com/in/sugamkuber))
