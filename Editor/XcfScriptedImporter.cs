#if UNITY_2020_2_OR_NEWER
using UnityEditor.AssetImporters;
#else
using UnityEditor.Experimental.AssetImporters;
#endif

using System.Text;
using UnityEngine;

namespace UniXCF
{
    [ScriptedImporter(1, "xcf")]
    public class XcfScriptedImporter : ScriptedImporter
    {
        public override void OnImportAsset(AssetImportContext ctx)
        {
            var base64 = NativeMethods.bakeImage(this.assetPath);
            var imageBytes = System.Convert.FromBase64String(base64);

            var texture = new Texture2D(2, 2, TextureFormat.RGBA32, false);
            texture.LoadImage(imageBytes);

            ctx.AddObjectToAsset("texture", texture);
            ctx.SetMainObject(texture);
        }
    }
}
