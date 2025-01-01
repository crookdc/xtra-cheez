#version 330 core
in vec2 TextureCoordinate;

out vec4 FragColor;

uniform sampler2D albedo;

void main()
{
    FragColor = texture(albedo, TextureCoordinate);
    // FragColor = texture(ourTexture, TexCoord) * vec4(Color, 1.0);
    // FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}