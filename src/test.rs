//! テスト。

use crate::{
    cosmic::square::{Angle, Degree45Orthant, DictOrthant, RelAdr2D},
    law::speed_of_light::Nine299792458,
};

/// テスト。
pub fn test() {
    // 辞書象限のテスト
    {
        let mut ort = DictOrthant::from_file_and_rank(0, -1);
        test_dort("e1", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, -1);
        test_dort("e2", "IV", &ort);
        ort = DictOrthant::from_file_and_rank(1, 0);
        test_dort("e3", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, 1);
        test_dort("e4", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(0, 1);
        test_dort("e5", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 1);
        test_dort("e6", "II", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 0);
        test_dort("e7", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, -1);
        test_dort("e8", "IOrIII", &ort);
    }
    // 45°回転象限のテスト
    {
        // TODO speed_of_light に West とか相対座標を入れておきたい。
        let mut ort = Degree45Orthant::new(&RelAdr2D::new(0, -1));
        test_d45ort("f1", "CoIIIOrCoIV", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(1, -1));
        test_d45ort("f2", "IVOrI", &ort);
        ort = Degree45Orthant::new(&Nine299792458::west());
        test_d45ort("f3", "IVOrI", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(1, 1));
        test_d45ort("f4", "IVOrI", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(0, 1));
        test_d45ort("f5", "CoIOrCoII", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(-1, 1));
        test_d45ort("f6", "IIOrIII", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(-1, 0));
        test_d45ort("f7", "IIOrIII", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(-1, -1));
        test_d45ort("f8", "IIOrIII", &ort);
    }
    // 相対番地のテスト
    {
        test_rsq("b1", "(0x -1y -1adr)", &RelAdr2D::new(0, -1));
        test_rsq("b2", "(1x -1y 9adr)", &RelAdr2D::new(1, -1));
        test_rsq("b3", "(1x 0y 10adr)", &Nine299792458::west());
        test_rsq("b4", "(1x 1y 11adr)", &RelAdr2D::new(1, 1));
        test_rsq("b5", "(0x 1y 1adr)", &RelAdr2D::new(0, 1));
        test_rsq("b6", "(-1x 1y -9adr)", &RelAdr2D::new(-1, 1));
        test_rsq("b7", "(-1x 0y -10adr)", &RelAdr2D::new(-1, 0));
        test_rsq("b8", "(-1x -1y -11adr)", &RelAdr2D::new(-1, -1));
    }
    // 45°回転のテスト
    {
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("a1", "(0x -1y -1adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a2", "(1x -1y 9adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a3", "(1x 0y 10adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a4", "(1x 1y 11adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a5", "(0x 1y 1adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a6", "(-1x 1y -9adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a7", "(-1x 0y -10adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a8", "(-1x -1y -11adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a9", "(0x -1y -1adr)", &r);
    }
    // 90°回転のテスト＜その１＞
    {
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("c1", "(0x -1y -1adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c2", "(1x 0y 10adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c3", "(0x 1y 1adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c4", "(-1x 0y -10adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c5", "(0x -1y -1adr)", &r);
    }
    // 90°回転のテスト＜その２＞
    {
        let mut r = RelAdr2D::new(1, -1);
        test_rsq("d1", "(1x -1y 9adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d2", "(1x 1y 11adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d3", "(-1x 1y -9adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d4", "(-1x -1y -11adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d5", "(1x -1y 9adr)", &r);
    }
    // 桂馬のテスト
    {
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("g1", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw45);
        test_rsq("g2", "(1x -1y 9adr)", &r);
        r.double_rank();
        test_rsq("g3", "(1x -2y 8adr)", &r);

        let mut r = RelAdr2D::new(0, -1);
        test_rsq("g4", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw315);
        test_rsq("g5", "(-1x -1y -11adr)", &r);
        r.double_rank();
        test_rsq("g6", "(-1x -2y -12adr)", &r);

        let mut r = RelAdr2D::new(0, 1);
        test_rsq("g7", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw45);
        test_rsq("g8", "(-1x 1y -9adr)", &r);
        r.double_rank();
        test_rsq("g9", "(-1x 2y -8adr)", &r);

        let mut r = RelAdr2D::new(0, 1);
        test_rsq("g10", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw315);
        test_rsq("g11", "(1x 1y 11adr)", &r);
        r.double_rank();
        test_rsq("g12", "(1x 2y 12adr)", &r);
    }
    // 角度指定回転のテスト(北から)
    {
        // 0
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("h1", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x -1y -1adr)", &r);

        // 45
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw45);
        test_rsq("h3", "(1x -1y 9adr)", &r);

        // 90
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw90);
        test_rsq("h4", "(1x 0y 10adr)", &r);

        // 135
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw135);
        test_rsq("h5", "(1x 1y 11adr)", &r);

        // 180
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x 1y 1adr)", &r);

        // 225
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw225);
        test_rsq("h7", "(-1x 1y -9adr)", &r);

        // 270
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw270);
        test_rsq("h8", "(-1x 0y -10adr)", &r);

        // 315
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw315);
        test_rsq("h9", "(-1x -1y -11adr)", &r);
    }
    // 角度指定回転のテスト(南から)
    {
        // 0
        let mut r = RelAdr2D::new(0, 1);
        test_rsq("h1", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x 1y 1adr)", &r);

        // 45
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw45);
        test_rsq("h3", "(-1x 1y -9adr)", &r);

        // 90
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw90);
        test_rsq("h4", "(-1x 0y -10adr)", &r);

        // 135
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw135);
        test_rsq("h5", "(-1x -1y -11adr)", &r);

        // 180
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x -1y -1adr)", &r);

        // 225
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw225);
        test_rsq("h7", "(1x -1y 9adr)", &r);

        // 270
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw270);
        test_rsq("h8", "(1x 0y 10adr)", &r);

        // 315
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw315);
        test_rsq("h9", "(1x 1y 11adr)", &r);
    }
}

fn test_dort(test_name: &str, expected: &str, actual: &DictOrthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_d45ort(test_name: &str, expected: &str, actual: &Degree45Orthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_rsq(test_name: &str, expected: &str, actual: &RelAdr2D) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
