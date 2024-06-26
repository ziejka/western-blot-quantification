export type Sample = {
   membrane_title: String,
   name: String,
   is_reference: boolean,
   area: number[],
   mean_od: number[],
   blank: number[],
   norm_by_reference: number[],
   normalized: number[],
}
