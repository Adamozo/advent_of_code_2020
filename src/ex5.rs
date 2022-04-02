// wyciecie z wejscia pierwszych cześci od rzędów i części od kolumnt
// przjsice po znakach i zmapowanie ich jako ciągu liczb binarnych

/*

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

    row * 8 + column = seat id
*/

// decode_id(code: &str) - function that takes code and returns struc seat with fields row, column, id
// moża zrobić jednolinikowcami - slice, po nim iterator i mapownie na 0/1 albo map z foldem