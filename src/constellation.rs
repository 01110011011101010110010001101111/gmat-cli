use std::fs::File;
use std::io::Write;

pub fn create_constellation(i: i32, t: i32, p: i32, f: i32, _ecc: f64, _sma: f64, _aop: f64, script_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let ecc = _ecc;
    let inc = i;
    // radius of earth in km + sma
    let sma = 6371.0 + _sma;
    let aop = _aop;

    let mut final_script = String::new();

    for num in 0..t {
        let ta = f*360 / (t/p) * (num % (t/p));
        let raan = 360 / p * ((num / (t/p)));

        let spacecraft_definition = format!(
        r#"
Create Spacecraft Sat{};
GMAT Sat{}.DateFormat = UTCGregorian;
GMAT Sat{}.Epoch = '19 Jun 2023 20:59:04.000';
GMAT Sat{}.CoordinateSystem = EarthMJ2000Eq;
GMAT Sat{}.DisplayStateType = Keplerian;
GMAT Sat{}.SMA = {};
GMAT Sat{}.ECC = {};
GMAT Sat{}.INC = {};
GMAT Sat{}.RAAN = {};
GMAT Sat{}.AOP = {};
GMAT Sat{}.TA = {};
GMAT Sat{}.DryMass = 850;
GMAT Sat{}.Cd = 2.2;
GMAT Sat{}.Cr = 1.8;
GMAT Sat{}.DragArea = 15;
GMAT Sat{}.SRPArea = 1;
GMAT Sat{}.SPADDragScaleFactor = 1;
GMAT Sat{}.SPADSRPScaleFactor = 1;
GMAT Sat{}.AtmosDensityScaleFactor = 1;
GMAT Sat{}.ExtendedMassPropertiesModel = 'None';
GMAT Sat{}.NAIFId = -123456789;
GMAT Sat{}.NAIFIdReferenceFrame = -123456789;
GMAT Sat{}.OrbitColor = Red;
GMAT Sat{}.TargetColor = Teal;
GMAT Sat{}.OrbitErrorCovariance = [ 1e+70 0 0 0 0 0 ; 0 1e+70 0 0 0 0 ; 0 0 1e+70 0 0 0 ; 0 0 0 1e+70 0 0 ; 0 0 0 0 1e+70 0 ; 0 0 0 0 0 1e+70 ];
GMAT Sat{}.CdSigma = 1e+70;
GMAT Sat{}.CrSigma = 1e+70;
GMAT Sat{}.Id = 'SatId';
GMAT Sat{}.Attitude = CoordinateSystemFixed;
GMAT Sat{}.SPADSRPInterpolationMethod = Bilinear;
GMAT Sat{}.SPADSRPScaleFactorSigma = 1e+70;
GMAT Sat{}.SPADDragInterpolationMethod = Bilinear;
GMAT Sat{}.SPADDragScaleFactorSigma = 1e+70;
GMAT Sat{}.AtmosDensityScaleFactorSigma = 1e+70;
GMAT Sat{}.ModelFile = '../data/vehicle/models/aura.3ds';
GMAT Sat{}.ModelOffsetX = 0;
GMAT Sat{}.ModelOffsetY = 0;
GMAT Sat{}.ModelOffsetZ = 0;
GMAT Sat{}.ModelRotationX = 0;
GMAT Sat{}.ModelRotationY = 0;
GMAT Sat{}.ModelRotationZ = 0;
GMAT Sat{}.ModelScale = 1.2;
GMAT Sat{}.AttitudeDisplayStateType = 'Quaternion';
GMAT Sat{}.AttitudeRateDisplayStateType = 'AngularVelocity';
GMAT Sat{}.AttitudeCoordinateSystem = EarthMJ2000Eq;
GMAT Sat{}.EulerAngleSequence = '321';
        "#,
        num, num, num, num, num, num, sma, num, ecc, num, inc, num, raan, num, aop, num, ta, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num, num
        );

        final_script.push_str(&spacecraft_definition);

    }

    let sat_set: String = (0..t)
        // .iter()
        .map(|sat| format!("Sat{}", sat))
        .collect::<Vec<String>>()
        .join(", ");

    let sat_alt: String = (0..t)
        // .iter()
        .map(|sat| format!("Sat{}.Earth.Altitude", sat))
        .collect::<Vec<String>>()
        .join(", ");

    let rest_of_script = format!(
    r#"
%----------------------------------------
%---------- Formation
%----------------------------------------

Create Formation form;
GMAT form.Add = {{ {} }};

%----------------------------------------
%---------- ForceModels
%----------------------------------------

Create ForceModel fm;
GMAT fm.CentralBody = Earth;
GMAT fm.PointMasses = {{ Earth, Sun, Luna }};
GMAT fm.Drag = None;
GMAT fm.SRP = Off;
GMAT fm.RelativisticCorrection = Off;
GMAT fm.ErrorControl = RSSStep;

%----------------------------------------
%---------- Propagators
%----------------------------------------

Create Propagator prop;
GMAT prop.FM = fm;
GMAT prop.Type = RungeKutta89;
GMAT prop.InitialStepSize = 60;
GMAT prop.Accuracy = 9.999999999999999e-12;
GMAT prop.MinStep = 0.001;
GMAT prop.MaxStep = 2700;
GMAT prop.MaxStepAttempts = 50;
GMAT prop.StopIfAccuracyIsViolated = true;

Create OrbitView OrbitView1;
GMAT OrbitView1.SolverIterations = Current;
GMAT OrbitView1.UpperLeft = [ 0 0 ];
GMAT OrbitView1.Size = [ 0 0 ];
GMAT OrbitView1.RelativeZOrder = 0;
GMAT OrbitView1.Maximized = false;
GMAT OrbitView1.Add = {{ {}, Earth }};
GMAT OrbitView1.CoordinateSystem = EarthMJ2000Eq;
GMAT OrbitView1.DrawObject = [ true true true ];
GMAT OrbitView1.DataCollectFrequency = 1;
GMAT OrbitView1.UpdatePlotFrequency = 50;
GMAT OrbitView1.NumPointsToRedraw = 0;
GMAT OrbitView1.ShowPlot = true;
GMAT OrbitView1.MaxPlotPoints = 20000;
GMAT OrbitView1.ShowLabels = true;
GMAT OrbitView1.ViewPointReference = Earth;
GMAT OrbitView1.ViewPointVector = [ 0 0 30000 ];
GMAT OrbitView1.ViewDirection = Earth;
GMAT OrbitView1.ViewScaleFactor = 1;
GMAT OrbitView1.ViewUpCoordinateSystem = EarthMJ2000Eq;
GMAT OrbitView1.ViewUpAxis = Z;
GMAT OrbitView1.EclipticPlane = Off;
GMAT OrbitView1.XYPlane = On;
GMAT OrbitView1.WireFrame = Off;
GMAT OrbitView1.Axes = On;
GMAT OrbitView1.Grid = Off;
GMAT OrbitView1.SunLine = Off;
GMAT OrbitView1.UseInitialView = On;
GMAT OrbitView1.StarCount = 7000;
GMAT OrbitView1.EnableStars = On;
GMAT OrbitView1.EnableConstellations = On;

Create ReportFile ReportFile1;
GMAT ReportFile1.SolverIterations = Current;
GMAT ReportFile1.UpperLeft = [ 0 0 ];
GMAT ReportFile1.Size = [ 0 0 ];
GMAT ReportFile1.RelativeZOrder = 0;
GMAT ReportFile1.Maximized = false;
GMAT ReportFile1.Filename = 'ReportFile1.tsv';
GMAT ReportFile1.Precision = 16;
GMAT ReportFile1.Add = {{ {} }};
GMAT ReportFile1.WriteHeaders = true;
GMAT ReportFile1.LeftJustify = On;
GMAT ReportFile1.ZeroFill = Off;
GMAT ReportFile1.FixedWidth = false;
GMAT ReportFile1.Delimiter = '	';
GMAT ReportFile1.ColumnWidth = 23;
GMAT ReportFile1.WriteReport = true;

%----------------------------------------
%---------- Mission Sequence
%----------------------------------------


BeginMissionSequence;

Propagate 'Prop 1 Day' prop(form) {{ Sat{}.ElapsedSecs = 86400.0 }};

    "#,
    sat_set,
    sat_set,
    sat_alt,
    "0", // have the 0th as the reference
    );

    final_script.push_str(&rest_of_script);

    // Save the file
    let mut file = File::create(script_name)?;
    file.write_all(final_script.as_bytes())?;

    Ok(())
}
