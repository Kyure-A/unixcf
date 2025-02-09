#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;
using UnityEngine;

namespace UniXCF
{
    public static class NativeMethods
    {
        [DllImport("native", EntryPoint = "bakeImage", CharSet = CharSet.Ansi)]
        private static extern IntPtr bakeImageInternal(string path);

        public static string bakeImage(string path)
        {
            IntPtr ptr = bakeImageInternal(path);
            if (ptr == IntPtr.Zero) return null; // 失敗した場合

            string result = Marshal.PtrToStringAnsi(ptr);
            Marshal.FreeHGlobal(ptr);
            return result;
        }
    }
}
