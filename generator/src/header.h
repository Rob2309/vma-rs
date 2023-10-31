
#define VMA_LEN_IF_NOT_NULL(len) __attribute__((annotate("LEN:"#len)))
#define VMA_EXTENDS_VK_STRUCT(vkStruct) __attribute__((annotate("VK_STRUCT:"#vkStruct)))
#include <vk_mem_alloc.h>
