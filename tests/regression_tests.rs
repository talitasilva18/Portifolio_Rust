use timewise_analytics::*;

#[test]
fn test_regressao_linear_simples() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [2.0, 4.0, 6.0, 8.0, 10.0];

    let resultado = regressao_linear(&x, &y);
    assert!(resultado.is_some());

    let (a, b) = resultado.unwrap();


    assert!((a - 2.0).abs() < 0.0001);
    assert!((b - 0.0).abs() < 0.0001);
}

#[test]
fn test_mse_calculo() {
    let y_real = [3.0, 5.0, 7.0];
    let y_predito = [2.9, 5.1, 6.8];

    let mse = calcular_mse(&y_real, &y_predito);

    let esperado = 0.02;
    assert!((mse - esperado).abs() < 0.000001); 
}


#[test]
fn test_r2_perfeito() {
    let y_real = [1.0, 2.0, 3.0, 4.0];
    let y_predito = [1.0, 2.0, 3.0, 4.0];

    let r2 = calcular_r2(&y_real, &y_predito);
    assert!((r2 - 1.0).abs() < 0.0001); 
}

#[test]
fn test_prever_valores() {
    let x = [6.0, 7.0, 8.0];
    let coeficientes = (2.0, 0.0); // y = 2x

    let esperado = vec![12.0, 14.0, 16.0];
    let resultado = prever(&x, coeficientes);

    for (a, b) in resultado.iter().zip(esperado.iter()) {
        assert!((*a - *b).abs() < 0.0001);
    }
}

#[test]
fn test_regressao_com_array_vazio() {
    let x: [f64; 0] = [];
    let y: [f64; 0] = [];

    let resultado = regressao_linear(&x, &y);
    assert!(resultado.is_none()); // Deve retornar None por falta de dados
}
