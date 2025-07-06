#include "c_api.h"

// Additional includes for complete NCNN functionality
//#include "allocator.h"
//#include "datareader.h"
//#include "mat.h"
//#include "net.h"
//#include "option.h"
//#include "layer.h"
//#include "blob.h"
//#include "paramdict.h"

// Platform specific includes
#if NCNN_VULKAN
#include "gpu.h"
//#include "command.h"
#endif
