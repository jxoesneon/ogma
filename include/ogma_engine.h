#ifndef OGMA_ENGINE_H
#define OGMA_ENGINE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OgmaEngine OgmaEngine;

OgmaEngine* ogma_engine_new(void);
void ogma_engine_free(OgmaEngine* engine_ptr);
char* ogma_engine_pre_seed(
    const OgmaEngine* engine_ptr,
    const char* prompt_ptr,
    const char* domain_ptr,
    const char* model_ptr
);
char* ogma_engine_analyze_json(const OgmaEngine* engine_ptr, const char* text_ptr);
char* ogma_engine_surgical_fix_json(const OgmaEngine* engine_ptr, const char* text_ptr);
char* ogma_engine_converge_json(const OgmaEngine* engine_ptr, const char* text_ptr);
char* ogma_engine_sarif_json(
    const OgmaEngine* engine_ptr,
    const char* text_ptr,
    const char* artifact_uri_ptr
);
void ogma_engine_free_str(char* s_ptr);

#ifdef __cplusplus
}
#endif

#endif /* OGMA_ENGINE_H */
