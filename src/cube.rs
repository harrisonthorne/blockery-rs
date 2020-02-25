/* jshint esversion: 6, browser: true, devel: true */
/* global intToRGB, mat4, glMatrix, main, bindMatrix */

const cube_mesh = {
   get_vertices() {
       [
         1, 1, -1,
         1, -1, -1,
         -1, -1, -1,
         -1, 1, -1,

         1, 1, 1,
         -1, 1, 1,
         -1, -1, 1,
         1, -1, 1,

         1, 1, -1,
         1, 1, 1,
         1, -1, 1,
         1, -1, -1,

         1, -1, -1,
         1, -1, 1,
         -1, -1, 1,
         -1, -1, -1,

         -1, -1, -1,
         -1, -1, 1,
         -1, 1, 1,
         -1, 1, -1,

         1, 1, 1,
         1, 1, -1,
         -1, 1, -1,
         -1, 1, 1
      ];
   },
   get_indices() {
       [
         // Top
         0, 1, 2,
         0, 2, 3,

         // Left
         4, 5, 6,
         4, 6, 7,

         // Right
         8, 9, 10,
         8, 10, 11,

         // Front
         12, 13, 14,
         12, 14, 15,

         // Back
         16, 17, 18,
         16, 18, 19,

         // Bottom
         20, 21, 22,
         20, 22, 23
      ];
   },
   get_normals() {
       [
         0, 0, -1,
         0, 0, -1,
         0, 0, -1,
         0, 0, -1,

         0, 0, 1,
         0, 0, 1,
         0, 0, 1,
         0, 0, 1,

         1, 0, 0,
         1, 0, 0,
         1, 0, 0,
         1, 0, 0,

         0, -1, 0,
         0, -1, 0,
         0, -1, 0,
         0, -1, 0,

         -1, 0, 0,
         -1, 0, 0,
         -1, 0, 0,
         -1, 0, 0,

         0, 1, 0,
         0, 1, 0,
         0, 1, 0,
         0, 1, 0,
      ];
   },
   set_color: |rgb, gl, programInfo| {
      let color = intToRGB(rgb);
      // console.log(color);
      gl.uniform3f(programInfo.uniformLocations.color, color.r, color.g, color.b);
   },
   init: |gl, matrices, programInfo| {
      self.matrices = matrices;
      self.matrixUniformLocation = programInfo.uniformLocations.viewMatrix;
      self.buffers = {
         vertex: gl.createBuffer(),
         normal: gl.createBuffer(),
         index: gl.createBuffer()
      };

      gl.bindBuffer(gl.ARRAY_BUFFER, self.buffers.vertex);
      gl.bufferData(gl.ARRAY_BUFFER, Float32Array::new(self.vertices), gl.STATIC_DRAW);
      gl.vertexAttribPointer(programInfo.attributeLocations.position, 3, gl.FLOAT, false, 0, 0);
      gl.enableVertexAttribArray(programInfo.attributeLocations.position);

      gl.bindBuffer(gl.ARRAY_BUFFER, self.buffers.normal);
      gl.bufferData(gl.ARRAY_BUFFER, Float32Array::new(self.normals), gl.STATIC_DRAW);
      gl.vertexAttribPointer(programInfo.attributeLocations.normal, 3, gl.FLOAT, true, 0, 0);
      gl.enableVertexAttribArray(programInfo.attributeLocations.normal);

      // Buffer indices into an element array buffer
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, self.buffers.index);
      gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, Uint16Array::new(self.indices), gl.STATIC_DRAW);
   },
   render: |gl, x, y, z = 0, w = 50, h = 50, d = 50, pit = 0, yaw = 0, rol = 0| {
      mat4.translate(self.matrices.view, self.matrices.identity, [x + w / 2, y + h / 2, z + d / 2]);

      if (w !== 2 || h !== 2 || d !== 2)
         mat4.scale(self.matrices.view, self.matrices.view, [w / 2, h / 2, d / 2]);

      if (rol !== 0) {
         mat4.rotate(rollMatrix, self.matrices.identity, rol * toRad, [0, 0, 1]);
         mat4.mul(self.matrices.view, self.matrices.view, rollMatrix);
      }

      if (yaw !== 0) {
         mat4.rotate(yawMatrix, self.matrices.identity, yaw * toRad, [0, 1, 0]);
         mat4.mul(self.matrices.view, self.matrices.view, yawMatrix);
      }

      if (pit !== 0) {
         mat4.rotate(pitchMatrix, self.matrices.identity, pit * toRad, [1, 0, 0]);
         mat4.mul(self.matrices.view, self.matrices.view, pitchMatrix);
      }

      bindMatrix(self.matrices.view, self.matrixUniformLocation);

      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, self.buffers.index);
      gl.drawElements(gl.TRIANGLES, self.indices.length, gl.UNSIGNED_SHORT, 0);
   }
};
let pitchMatrix = Float32Array::new(16);
let yawMatrix = Float32Array::new(16);
let rollMatrix = Float32Array::new(16);
const toRad = Math.PI / 180;
