use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

// This is an obnoxiously long table, so we will use a macro here.
macro_rules! constant_table {
    ( $( $ty:ty => $name:ident ),* ) => {
        #[derive(Debug, Clone)]
        pub struct MathConstantTable {
            $( pub $name: $ty, )*
        }

        impl MathConstantTable {
            pub fn read_table<T: Read>(rdr: &mut T) -> Result<MathConstantTable, String> {
                Ok(MathConstantTable{
                    $( $name: <$ty as Value>::read_value(rdr), )*
                })
            } 
        }
    };
}

trait Value {
    fn read_value<T: Read>(&mut T) -> Self;
}

impl Value for i16 {
    fn read_value<T: Read>(rdr: &mut T) -> i16 {
        rdr.read_i16::<BigEndian>().unwrap()
    }
}

impl Value for u16 {
    fn read_value<T: Read>(rdr: &mut T) -> u16 {
        rdr.read_u16::<BigEndian>().unwrap()
    }
}

impl Value for MathValueRecord {
    fn read_value<T: Read>(rdr: &mut T) -> MathValueRecord {
        MathValueRecord {
            value: rdr.read_i16::<BigEndian>().unwrap(),
            device_table: rdr.read_u16::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MathValueRecord {
    pub value: i16,
    pub device_table: u16,
}

constant_table!{
    i16             => script_percent_scale_down,
    i16             => script_script_percent_scale_down,
    u16             => delimited_sub_formula_min_height,
    u16             => display_operator_min_height,
    MathValueRecord => math_leading,
    MathValueRecord => axis_height,
    MathValueRecord => accent_base_height,
    MathValueRecord => flattened_accent_base_height,
    MathValueRecord => subscript_shift_down,
    MathValueRecord => subscript_top_max,
    MathValueRecord => subscript_baseline_drop_min,
    MathValueRecord => superscript_shift_up,
    MathValueRecord => superscript_shift_up_cramped,
    MathValueRecord => superscript_bottom_min,
    MathValueRecord => superscript_baseline_drop_max,
    MathValueRecord => sub_superscript_gap_min,
    MathValueRecord => superscript_bottom_max_with_subscript,
    MathValueRecord => space_after_script,
    MathValueRecord => upper_limit_gap_min,
    MathValueRecord => upper_limit_baseline_rise_min,
    MathValueRecord => lower_limit_gap_min,
    MathValueRecord => lower_limit_baseline_drop_min,
    MathValueRecord => stack_top_shift_up,
    MathValueRecord => stack_top_display_style_shift_up,
    MathValueRecord => stack_bottom_shift_down,
    MathValueRecord => stack_bottom_display_style_shift_down,
    MathValueRecord => stack_gap_min,
    MathValueRecord => stack_display_style_gap_min,
    MathValueRecord => stretch_stack_top_shift_up,
    MathValueRecord => stretch_stack_bottom_shift_down,
    MathValueRecord => stretch_stack_gap_above_min,
    MathValueRecord => stretch_stack_gap_below_min,
    MathValueRecord => fraction_numerator_shift_up,
    MathValueRecord => fraction_numerator_display_style_shift_up,
    MathValueRecord => fraction_denominator_shift_down,
    MathValueRecord => fraction_denominator_display_style_shift_down,
    MathValueRecord => fraction_numerator_gap_min,
    MathValueRecord => fraction_num_display_style_gap_min,
    MathValueRecord => fraction_rule_thickness,
    MathValueRecord => fraction_denominator_gap_min,
    MathValueRecord => fraction_denom_display_style_gap_min,
    MathValueRecord => skewed_fraction_horizontal_gap,
    MathValueRecord => skewed_fraction_vertical_gap,
    MathValueRecord => overbar_vertical_gap,
    MathValueRecord => overbar_rule_thickness,
    MathValueRecord => overbar_extra_ascender,
    MathValueRecord => underbar_vertical_gap,
    MathValueRecord => underbar_rule_thickness,
    MathValueRecord => underbar_extra_descender,
    MathValueRecord => radical_vertical_gap,
    MathValueRecord => radical_display_style_vertical_gap,
    MathValueRecord => radical_rule_thickness,
    MathValueRecord => radical_extra_ascender,
    MathValueRecord => radical_kern_before_degree,
    MathValueRecord => radical_kern_after_degree,
    i16             => radical_degree_bottom_raise_percent
}