use ml::gd::gd;
use ml::tensor::Tensor;

fn main() {
    let x = Tensor::new(
        vec![100, 4],
        vec![
            44.0, 39.0, 23.0, 32.0, 39.0, 47.0, 25.0, 42.0, 15.0, 17.0, 3.0, 2.0, 26.0, 38.0, 23.0,
            45.0, 41.0, 44.0, 26.0, 4.0, 27.0, 6.0, 48.0, 32.0, 18.0, 32.0, 8.0, 14.0, 2.0, 4.0,
            33.0, 6.0, 8.0, 23.0, 34.0, 10.0, 45.0, 16.0, 28.0, 45.0, 46.0, 3.0, 45.0, 47.0, 18.0,
            23.0, 22.0, 11.0, 24.0, 46.0, 37.0, 15.0, 26.0, 28.0, 46.0, 19.0, 6.0, 37.0, 31.0,
            22.0, 47.0, 39.0, 18.0, 4.0, 20.0, 11.0, 47.0, 39.0, 7.0, 22.0, 18.0, 20.0, 37.0, 10.0,
            43.0, 2.0, 26.0, 29.0, 49.0, 2.0, 32.0, 25.0, 42.0, 21.0, 44.0, 37.0, 11.0, 12.0, 39.0,
            46.0, 21.0, 41.0, 6.0, 41.0, 27.0, 47.0, 16.0, 34.0, 8.0, 4.0, 12.0, 4.0, 5.0, 40.0,
            41.0, 15.0, 39.0, 14.0, 34.0, 8.0, 8.0, 2.0, 12.0, 43.0, 25.0, 9.0, 20.0, 8.0, 40.0,
            27.0, 44.0, 15.0, 27.0, 3.0, 15.0, 27.0, 6.0, 20.0, 29.0, 29.0, 20.0, 14.0, 48.0, 17.0,
            46.0, 44.0, 46.0, 48.0, 32.0, 43.0, 26.0, 43.0, 27.0, 43.0, 17.0, 44.0, 19.0, 16.0,
            17.0, 32.0, 27.0, 7.0, 22.0, 23.0, 24.0, 7.0, 46.0, 47.0, 6.0, 3.0, 32.0, 24.0, 19.0,
            19.0, 26.0, 33.0, 13.0, 8.0, 23.0, 17.0, 33.0, 18.0, 28.0, 26.0, 3.0, 3.0, 10.0, 32.0,
            37.0, 5.0, 48.0, 21.0, 49.0, 27.0, 46.0, 34.0, 44.0, 27.0, 12.0, 40.0, 23.0, 25.0,
            34.0, 4.0, 23.0, 29.0, 34.0, 38.0, 49.0, 16.0, 32.0, 43.0, 27.0, 10.0, 23.0, 5.0, 5.0,
            32.0, 2.0, 39.0, 42.0, 18.0, 14.0, 44.0, 45.0, 19.0, 49.0, 38.0, 9.0, 41.0, 22.0, 39.0,
            3.0, 24.0, 12.0, 35.0, 48.0, 6.0, 44.0, 17.0, 27.0, 25.0, 10.0, 20.0, 38.0, 14.0, 21.0,
            33.0, 35.0, 13.0, 2.0, 16.0, 27.0, 8.0, 31.0, 28.0, 33.0, 45.0, 29.0, 20.0, 19.0, 14.0,
            7.0, 38.0, 37.0, 30.0, 19.0, 29.0, 35.0, 24.0, 22.0, 31.0, 36.0, 7.0, 6.0, 11.0, 12.0,
            13.0, 47.0, 10.0, 13.0, 42.0, 47.0, 22.0, 44.0, 16.0, 23.0, 41.0, 40.0, 39.0, 43.0,
            23.0, 7.0, 26.0, 15.0, 29.0, 40.0, 30.0, 31.0, 32.0, 13.0, 6.0, 9.0, 17.0, 11.0, 13.0,
            23.0, 17.0, 31.0, 46.0, 3.0, 18.0, 10.0, 37.0, 20.0, 8.0, 46.0, 2.0, 45.0, 46.0, 22.0,
            24.0, 32.0, 33.0, 13.0, 39.0, 25.0, 3.0, 16.0, 40.0, 20.0, 49.0, 36.0, 2.0, 13.0, 35.0,
            22.0, 9.0, 15.0, 48.0, 48.0, 47.0, 5.0, 21.0, 29.0, 4.0, 26.0, 36.0, 10.0, 6.0, 36.0,
            24.0, 47.0, 12.0, 15.0, 43.0, 32.0, 21.0, 18.0, 27.0, 30.0, 19.0, 33.0, 38.0, 7.0,
            17.0, 27.0, 17.0, 24.0, 42.0, 3.0, 13.0, 27.0, 17.0, 31.0, 23.0, 49.0, 32.0, 47.0,
            31.0, 29.0, 17.0, 19.0, 38.0, 15.0, 11.0, 41.0, 19.0, 38.0, 26.0, 40.0, 10.0, 49.0,
            37.0, 20.0, 17.0, 22.0, 39.0, 22.0, 39.0, 33.0, 14.0, 14.0, 14.0, 45.0, 45.0, 43.0,
            14.0, 39.0, 16.0,
        ],
    );

    let mut w = Tensor::new(vec![4, 1], vec![0.0, 0.0, 0.0, 0.0]);

    let y = Tensor::new(
        vec![100, 1],
        vec![
            2587.0, 3184.0, 418.0, 3135.0, 1271.0, 2431.0, 1278.0, 787.0, 1258.0, 3031.0, 3201.0,
            1202.0, 1903.0, 2014.0, 1977.0, 1137.0, 2793.0, 1533.0, 964.0, 1215.0, 2056.0, 1407.0,
            3070.0, 3223.0, 792.0, 2173.0, 1592.0, 470.0, 1351.0, 2066.0, 901.0, 1477.0, 1453.0,
            3242.0, 3372.0, 3147.0, 1660.0, 1170.0, 1048.0, 1022.0, 1645.0, 1095.0, 1648.0, 641.0,
            1165.0, 2479.0, 2560.0, 2089.0, 1967.0, 2063.0, 1527.0, 1840.0, 1924.0, 2133.0, 2868.0,
            1817.0, 1404.0, 2025.0, 1484.0, 1606.0, 953.0, 3170.0, 1332.0, 2472.0, 2098.0, 1300.0,
            968.0, 2624.0, 1871.0, 3077.0, 1882.0, 2443.0, 1008.0, 1042.0, 3022.0, 2211.0, 894.0,
            2263.0, 2675.0, 2369.0, 1256.0, 1221.0, 3625.0, 854.0, 992.0, 1679.0, 2057.0, 1754.0,
            1562.0, 2751.0, 1372.0, 2668.0, 1834.0, 1296.0, 2227.0, 2807.0, 2540.0, 1707.0, 3073.0,
            1690.0,
        ],
    );

    let l = 0.0001;

    let e = 5000;

    for _ in 0..e {
        w = gd(&x, &y, &w, l)
    }

    println!("{:?}", w); // Should return something close to [5, 12, 13, 50]
}
