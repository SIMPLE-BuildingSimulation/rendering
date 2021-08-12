use crate::Float;
use crate::sampleable_trait::Sampleable;
use crate::colour::Spectrum;

// Flags
const DELTA_POSITION_LIGHT: u8 = 1;
const DELTA_DIRECTION_LIGHT : u8 = 2;
const AREA_LIGHT : u8 = 4;
const INFINITE_LIGHT : u8 = 8;

trait Light : Sampleable {    
    
    /// Returns the light flags associated with the 
    /// light source
    fn flags(&self)->u8;


    /// Returns the number of samples that should be used for this
    /// specific light source
    fn n_samples(&self)->usize;

    /// Checks whether a light source has a Dirac's delta position
    /// or direction
    fn is_delta_light(&self)->bool{        
        self.flags() & DELTA_POSITION_LIGHT == 1 ||
        self.flags() & DELTA_DIRECTION_LIGHT == 1        
    }

    // fn power(&self)->Float;
    
    
    fn sample_li(&self)->Spectrum;
    fn pdf_li(&self)->Float;

    fn le(&self)->Spectrum;
    fn sample_le(&self)->Spectrum;
    fn pdf_le(&self)->Float;
}




