#include <stdio.h>
#include <cuda.h>
 

__global__ void vecmul(float *A, float* B, float *C, int size) {
    // Row and Column indexes: 
    int row = blockIdx.y*blockDim.y+threadIdx.y;
    int col = blockIdx.x*blockDim.x+threadIdx.x;

    // Are they bellow the maximum?
    if (col < size && row < size) {
       float result = 0;
       for(int ix=0;ix<size;ix++) {
          result += A[row*size+ix]*B[ix*size+col];
       }
       C[row*size+col] = result;
    }
}


// Runs one connection, taking a value from a source node, multiplying it, and adding it to a destination node
__global__ void connection(float* mult, uint32_t* source, uint32_t* dest, float* output, uint32_t offset) {
    int id = threadIdx.x;
    atomicAdd(&output[dest[offset + id]], mult[offset + id] * output[source[offset + id]]);
}


// Normalizes a node's value so it can be used as a source node
__global__ void normalize(float* output, uint32_t offset) {
    int id = threadIdx.x;
    output[offset + id] = tanh(output[offset + id]);
}




extern "C" {
    // Verify that CUDA is available for use.
    uint32_t verify_cuda() {
        int devices = 0; 
        cudaError_t err = cudaGetDeviceCount(&devices); 
        return devices > 0 && err == cudaSuccess;
    }

    void maxmul(float *A, float* B, float *C, int size) {

        int total = size*size;

        // Allocate device memory:
        float* gpu_A;
        float* gpu_B;
        float* gpu_C;
        int msize = total * sizeof(float);
        cudaMalloc((void**)&gpu_A, msize);
        cudaMemcpy(gpu_A,A,msize,cudaMemcpyHostToDevice);
        cudaMalloc((void**)&gpu_B, msize);
        cudaMemcpy(gpu_B,B,msize,cudaMemcpyHostToDevice);
        cudaMalloc((void**)&gpu_C,msize);

        // Blocks & grids:
        dim3 blocks(size,size);
        dim3 grid(1,1);

        // Call the kernel:
        vecmul<<<grid,blocks>>>(gpu_A,gpu_B,gpu_C,size);

        // Get the result Matrix:
        cudaMemcpy(C,gpu_C,msize,cudaMemcpyDeviceToHost);

        C[0] = 2.3;

        //Free device matrices
        cudaFree(gpu_A);
        cudaFree(gpu_B);
        cudaFree(gpu_C);
    }


    // Calculates the output of the given neural network arrays
    void calculate(
        float* mult,
        uint32_t* source,
        uint32_t* dest,
        float* output,
        uint32_t* mult_threads,
        uint32_t* output_threads,

        uint32_t connections_size,
        uint32_t output_size,
        uint32_t threads_size
    ) {

        // allocate arrays onto vram
        float* g_mult;
        uint32_t* g_source;
        uint32_t* g_dest;
        float* g_output;
        cudaMalloc((void**)&g_mult, connections_size);
        cudaMemcpy(g_mult, mult, connections_size, cudaMemcpyHostToDevice);
        cudaMalloc((void**)&g_source, connections_size);
        cudaMemcpy(g_source, source, connections_size, cudaMemcpyHostToDevice);
        cudaMalloc((void**)&g_dest, connections_size);
        cudaMemcpy(g_dest, dest, connections_size, cudaMemcpyHostToDevice);
        cudaMalloc((void**)&g_output, output_size);
        cudaMemcpy(g_output, output, output_size, cudaMemcpyHostToDevice);

        uint32_t mult_offset = 0;
        uint32_t output_offset = 0;

        // for every layer
        for (int i = 0; i < threads_size; i++) {
            connection<<<1, mult_threads[i]>>>(g_mult, g_source, g_dest, g_output, mult_offset);
            mult_offset += mult_threads[i];
            normalize<<<1, output_threads[i]>>>(g_output, output_offset);
            output_offset += output_threads[i];
        }

    }

    /*
        mult: *const f32,
        source: *const usize,
        dest: *const usize,
        output: *mut f32,
        mult_threads: *const usize,
        output_threads: *const usize,

   */
}


