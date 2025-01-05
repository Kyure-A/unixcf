using UnityEngine;
#if UNITY_2020_2_OR_NEWER
using UnityEditor.AssetImporters;
#else
using UnityEditor.Experimental.AssetImporters;
#endif
using System;
using System.Text;

namespace UniXCF
{
    [ScriptedImporter(1, "xcf")]
    public class XcfScriptedImporter : ScriptedImporter
    {
        public override void OnImportAsset(AssetImportContext ctx)
        {
            var base64 = Native.bakeImage(this.assetPath);
            var str = Encoding.UTF8.GetString(base64);
        }
    }
}
