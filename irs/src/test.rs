use libflate::zlib::Decoder;
use simple_pdf_decrypt::KeyManager;
use simple_pdf_parser::Pdf;
use std::io::Read;

pub static PDF_FILE: &[u8] = include_bytes!("../account.pdf");

#[test]
pub fn test() {
    let mut pdf = Pdf::new(PDF_FILE);
    pdf.parse_trailer();
    pdf.find_encrypt_object();
    pdf.read_encrypt_object();

    let mut key_manager = KeyManager::new(&pdf);
    key_manager.compute_workspace_key();

    let ciphertext_offset = pdf.find_specific_obj(7);
    let ciphertext = pdf.find_stream(ciphertext_offset);

    let plaintext = KeyManager::decrypt(&key_manager.compute_object_key(7, 0), &ciphertext);

    let mut decoder = Decoder::new(&plaintext[..]).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    let string = String::from_utf8(decoded_data).unwrap();

    let find_data =
        |start_offset: usize, start_word: String, end_word: String| -> (usize, String) {
            let start = string[start_offset..].find(&start_word).unwrap();
            let end = string[start_offset + start + start_word.len()..]
                .find(&end_word)
                .unwrap();

            let new_start_offset = start_offset + start + start_word.len() + end;
            let string = String::from(&string[new_start_offset - end..new_start_offset]);

            (new_start_offset, string)
        };

    let (tax_period_start, tax_period) =
        find_data(0, "TAX PERIOD: ".parse().unwrap(), ")".parse().unwrap());
    assert_eq!(tax_period, "Dec. 31, 2022");

    let (_, taxable_income) = find_data(
        tax_period_start,
        "TAXABLE INCOME: ".parse().unwrap(),
        ")".parse().unwrap(),
    );
    let taxable_income = str::parse::<f64>(&taxable_income.trim().replace(',', "")).unwrap();
    assert!(taxable_income >= 200000.00);
}
