use std::f32::NAN;

use serde::Serialize;

use crate::models::sample::Sample;

#[derive(Serialize)]
pub struct Csv(pub Vec<u8>);

impl TryFrom<Vec<&Sample>> for Csv {
    type Error = csv::Error;

    fn try_from(samples: Vec<&Sample>) -> Result<Self, Self::Error> {
        let mut writer = csv::Writer::from_writer(Vec::new());
        let mut sorted_samples = samples.clone();
        sorted_samples.sort_by(|a, b| a.is_reference.cmp(&b.is_reference));

        for sample in sorted_samples.iter() {
            let header_row = vec![
                "Sample Name",
                "Area",
                "Mean OD",
                "Blank",
                "Norm by ref",
                "Normalized",
            ];
            let number_of_columns = header_row.len();
            let mut gen_name: Vec<&str> = vec![&sample.name];
            let empty_line = vec![""; number_of_columns];

            while gen_name.len() < number_of_columns {
                gen_name.push("")
            }
            writer.write_record(&gen_name)?;
            writer.write_record(&header_row)?;

            let mut row = vec![];
            for i in 0..sample.area.len() {
                row.push("".to_string());
                row.push(format!("{:.3}", sample.area[i]));
                row.push(format!("{:.3}", sample.mean_od[i]));
                row.push(format!("{:.3}", sample.blank[i]));
                row.push(format!(
                    "{:.3}",
                    sample.norm_by_reference.get(i).unwrap_or(&NAN)
                ));
                row.push(format!("{:.3}", sample.normalized.get(i).unwrap_or(&NAN)));

                writer.write_record(&row)?;
                row.clear();
            }
            writer.write_record(&empty_line)?;
        }

        let data = writer.into_inner().unwrap();
        Ok(Self(data))
    }
}
