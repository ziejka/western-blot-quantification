use serde::{Deserialize, Serialize};

use crate::SampleData;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Sample {
    pub document_title: String,
    pub name: String,
    pub is_reference: bool,
    pub area: Vec<f32>,
    pub mean_od: Vec<f32>,
    pub blank: Vec<f32>,
    pub norm_by_reference: Vec<f32>,
    pub normalized: Vec<f32>,
}

impl Sample {
    pub fn calculate_blank(&mut self) {
        if let Some(blank) = self.mean_od.last() {
            self.mean_od
                .iter()
                .for_each(|mean| self.blank.push(mean - *blank));
        }
    }

    pub fn calculate_norm_by(&mut self, reference_sample: &Sample) {
        self.blank
            .iter()
            .zip(reference_sample.blank.iter())
            .for_each(|(&blank, &ref_blank)| self.norm_by_reference.push(blank / ref_blank));
    }

    pub fn calculate_normalized(&mut self) {
        if let Some(control) = self.norm_by_reference.first() {
            self.norm_by_reference
                .iter()
                .for_each(|&norm_by_ref| self.normalized.push(norm_by_ref / *control));
        }
    }
}

impl TryFrom<SampleData> for Sample {
    type Error = &'static str;

    fn try_from(sd: SampleData) -> Result<Self, Self::Error> {
        let mut areas: Vec<f32> = Vec::new();
        let mut mean_ods: Vec<f32> = Vec::new();
        if sd.values.is_empty() {
            return Err("No experiment data");
        }
        for line in sd.values.trim().lines() {
            let mut split_iter = line.split("\t");

            let area: f32 = split_iter
                .nth(1)
                .ok_or("No value for area")?
                .replace(",", ".")
                .parse::<f32>()
                .map_err(|_| "area parse error")?;

            let mean_od: f32 = split_iter
                .next()
                .ok_or("No mean_od value")?
                .replace(",", ".")
                .parse()
                .map_err(|_| "mean_od value parse error")?;

            areas.push(area);
            mean_ods.push(mean_od);
        }

        Ok(Sample {
            document_title: sd.membrane_title,
            name: sd.name,
            is_reference: sd.is_reference,
            area: areas,
            mean_od: mean_ods,
            blank: vec![],
            norm_by_reference: vec![],
            normalized: vec![],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_test() {
        let input = SampleData {
            membrane_title: "membrane_title".into(),
            is_reference: false,
            name: "test".into(),
            values: "
            1	454.667	29263.027	0	30142
2	454.667	29263.027	0	30142
"
            .into(),
        };

        let result = Sample {
            document_title: "membrane_title".into(),
            name: "test".into(),
            is_reference: false,
            area: vec![454.667, 454.667],
            mean_od: vec![29263.027, 29263.027],
            blank: vec![],
            norm_by_reference: vec![],
            normalized: vec![],
        };

        assert_eq!(Ok(result), input.try_into());
    }
}
