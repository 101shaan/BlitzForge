#!/usr/bin/env python3
"""
BlitzForge Password Cracking Visualizer
Shows the POWER of password cracking with visual proof
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
from pathlib import Path
import sys

# set style for dramatic impact
sns.set_style("dark")
plt.rcParams['figure.facecolor'] = '#0a0a0a'
plt.rcParams['axes.facecolor'] = '#1a1a1a'
plt.rcParams['text.color'] = 'white'
plt.rcParams['axes.labelcolor'] = 'white'
plt.rcParams['xtick.color'] = 'white'
plt.rcParams['ytick.color'] = 'white'
plt.rcParams['grid.color'] = '#333333'
plt.rcParams['figure.figsize'] = (14, 10)
plt.rcParams['font.size'] = 12

def calculate_crack_times(hash_rate_per_sec):
    """
    calculate time to crack passwords of various lengths
    assumes lowercase + digits charset (36 chars)
    """
    charset_size = 36  # a-z + 0-9
    
    lengths = range(4, 13)  # 4 to 12 characters
    times = []
    
    for length in lengths:
        keyspace = charset_size ** length
        seconds = keyspace / hash_rate_per_sec
        times.append(seconds)
    
    return list(lengths), times

def format_time(seconds):
    """convert seconds to human readable format"""
    if seconds < 1:
        return f"{seconds*1000:.1f}ms"
    elif seconds < 60:
        return f"{seconds:.1f}s"
    elif seconds < 3600:
        return f"{seconds/60:.1f}m"
    elif seconds < 86400:
        return f"{seconds/3600:.1f}h"
    elif seconds < 31536000:
        return f"{seconds/86400:.1f}d"
    else:
        years = seconds / 31536000
        if years < 1000:
            return f"{years:.1f}y"
        elif years < 1000000:
            return f"{years/1000:.1f}K years"
        elif years < 1000000000:
            return f"{years/1000000:.1f}M years"
        else:
            return f"{years/1000000000:.1f}B years"

def plot_crack_time_by_length(hash_rate, output_path):
    """
    dramatic chart: time to crack vs password length
    """
    fig, ax = plt.subplots(figsize=(14, 10))
    
    lengths, times = calculate_crack_times(hash_rate)
    
    # plot with logarithmic scale
    colors = ['#00ff00' if t < 60 else '#ffaa00' if t < 86400 else '#ff0000' for t in times]
    bars = ax.bar(lengths, times, color=colors, edgecolor='white', linewidth=2)
    
    ax.set_yscale('log')
    ax.set_xlabel('Password Length (lowercase + digits)', fontsize=16, fontweight='bold')
    ax.set_ylabel('Time to Crack (seconds, log scale)', fontsize=16, fontweight='bold')
    ax.set_title(f'🔥 BlitzForge Cracking Power\n{hash_rate/1e9:.1f} Billion Hashes/Second', 
                 fontsize=20, fontweight='bold', color='#ff4444', pad=20)
    
    # add time labels on bars
    for i, (bar, time) in enumerate(zip(bars, times)):
        height = bar.get_height()
        label = format_time(time)
        ax.text(bar.get_x() + bar.get_width()/2., height*1.5,
                label, ha='center', va='bottom', fontsize=11, 
                fontweight='bold', color='white',
                bbox=dict(boxstyle='round,pad=0.5', facecolor='black', alpha=0.7))
    
    # add danger zones
    ax.axhline(y=1, color='#00ff00', linestyle='--', linewidth=2, alpha=0.5, label='< 1 second (INSTANT)')
    ax.axhline(y=3600, color='#ffaa00', linestyle='--', linewidth=2, alpha=0.5, label='< 1 hour (WEAK)')
    ax.axhline(y=86400, color='#ff6600', linestyle='--', linewidth=2, alpha=0.5, label='< 1 day (BAD)')
    
    ax.legend(loc='upper left', fontsize=12, framealpha=0.9)
    ax.grid(True, alpha=0.3)
    
    # add warning text
    fig.text(0.5, 0.02, 
             '⚠️  This is why you need 12+ character passwords! ⚠️',
             ha='center', fontsize=14, fontweight='bold', 
             color='#ff4444',
             bbox=dict(boxstyle='round,pad=1', facecolor='black', edgecolor='#ff4444', linewidth=3))
    
    plt.tight_layout(rect=[0, 0.03, 1, 1])
    plt.savefig(output_path, dpi=300, facecolor='#0a0a0a', edgecolor='none')
    print(f"📊 Saved: {output_path}")
    plt.close()

def plot_password_strength_comparison(hash_rate, output_path):
    """
    show common passwords vs strong passwords
    """
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 8))
    
    # weak passwords (found in top 10k list)
    weak_passwords = [
        ('password', 0.001),
        ('123456', 0.001),
        ('qwerty', 0.002),
        ('admin', 0.001),
        ('letmein', 0.003),
        ('password123', 0.005),
    ]
    
    weak_names = [p[0] for p in weak_passwords]
    weak_times = [p[1] for p in weak_passwords]
    
    bars1 = ax1.barh(weak_names, weak_times, color='#ff0000', edgecolor='white', linewidth=2)
    ax1.set_xlabel('Time to Crack (seconds)', fontsize=14, fontweight='bold')
    ax1.set_title('❌ WEAK Passwords\n(Dictionary Attack)', fontsize=16, fontweight='bold', color='#ff4444')
    ax1.set_xlim(0, max(weak_times)*1.3)
    
    # add time labels
    for i, (bar, time) in enumerate(zip(bars1, weak_times)):
        ax1.text(time*1.05, bar.get_y() + bar.get_height()/2,
                f'{time*1000:.1f}ms', va='center', fontsize=11, 
                fontweight='bold', color='white')
    
    ax1.text(0.5, -0.15, '⚠️ CRACKED INSTANTLY!', 
             transform=ax1.transAxes, ha='center',
             fontsize=14, fontweight='bold', color='#ff0000')
    
    # strong passwords
    strong_examples = [
        ('8 char random', 36**8),
        ('10 char random', 36**10),
        ('12 char random', 36**12),
        ('14 char random', 36**14),
        ('16 char random', 36**16),
    ]
    
    strong_names = [p[0] for p in strong_examples]
    strong_times = [p[1]/hash_rate for p in strong_examples]
    strong_labels = [format_time(t) for t in strong_times]
    
    colors = ['#ffaa00', '#ffaa00', '#00ff00', '#00ff00', '#00ff00']
    bars2 = ax2.barh(strong_names, strong_times, color=colors, edgecolor='white', linewidth=2)
    ax2.set_xlabel('Time to Crack (seconds, log scale)', fontsize=14, fontweight='bold')
    ax2.set_title('✅ STRONG Passwords\n(Brute Force)', fontsize=16, fontweight='bold', color='#44ff44')
    ax2.set_xscale('log')
    
    # add time labels
    for i, (bar, label) in enumerate(zip(bars2, strong_labels)):
        ax2.text(bar.get_width()*1.5, bar.get_y() + bar.get_height()/2,
                label, va='center', fontsize=11, 
                fontweight='bold', color='white',
                bbox=dict(boxstyle='round,pad=0.3', facecolor='black', alpha=0.7))
    
    ax2.text(0.5, -0.15, '✅ PRACTICALLY IMPOSSIBLE!', 
             transform=ax2.transAxes, ha='center',
             fontsize=14, fontweight='bold', color='#00ff00')
    
    fig.suptitle(f'Password Strength Comparison @ {hash_rate/1e9:.1f} GH/s', 
                 fontsize=20, fontweight='bold', y=0.98)
    
    plt.tight_layout(rect=[0, 0.03, 1, 0.95])
    plt.savefig(output_path, dpi=300, facecolor='#0a0a0a', edgecolor='none')
    print(f"📊 Saved: {output_path}")
    plt.close()

def plot_algorithm_comparison(hash_rate_blitz, output_path):
    """
    compare BlitzHash vs other algorithms
    """
    fig, ax = plt.subplots(figsize=(14, 10))
    
    algorithms = [
        ('BlitzHash\n(Custom)', hash_rate_blitz, '#ff4444'),
        ('MD5', hash_rate_blitz * 0.5, '#ff8844'),
        ('SHA1', hash_rate_blitz * 0.3, '#ffaa44'),
        ('SHA256', hash_rate_blitz * 0.15, '#ffcc44'),
        ('Argon2\n(Real Security)', 10, '#00ff00'),  # 10 H/s
    ]
    
    names = [a[0] for a in algorithms]
    rates = [a[1] for a in algorithms]
    colors = [a[2] for a in algorithms]
    
    bars = ax.bar(names, rates, color=colors, edgecolor='white', linewidth=3)
    ax.set_yscale('log')
    ax.set_ylabel('Hashes per Second (log scale)', fontsize=16, fontweight='bold')
    ax.set_title('🔥 Algorithm Speed Comparison\nWhy Real Systems Use Slow Hashing', 
                 fontsize=20, fontweight='bold', pad=20)
    
    # add rate labels
    for bar, rate in zip(bars, rates):
        height = bar.get_height()
        if rate >= 1e9:
            label = f'{rate/1e9:.1f} GH/s'
        elif rate >= 1e6:
            label = f'{rate/1e6:.0f} MH/s'
        else:
            label = f'{rate:.0f} H/s'
        
        ax.text(bar.get_x() + bar.get_width()/2., height*1.5,
                label, ha='center', va='bottom', fontsize=13, 
                fontweight='bold', color='white',
                bbox=dict(boxstyle='round,pad=0.5', facecolor='black', alpha=0.8))
    
    # add speedup multipliers
    base_rate = rates[-1]  # Argon2
    for i, (bar, rate) in enumerate(zip(bars[:-1], rates[:-1])):
        multiplier = rate / base_rate
        ax.text(bar.get_x() + bar.get_width()/2., 1,
                f'{multiplier/1e6:.0f}M× faster', ha='center', va='bottom', 
                fontsize=11, color='#ff4444', fontweight='bold',
                bbox=dict(boxstyle='round,pad=0.3', facecolor='black', alpha=0.7))
    
    ax.grid(True, alpha=0.3, axis='y')
    
    # add explanation
    fig.text(0.5, 0.02,
             '💡 Even BlitzHash at 5 GH/s is 500 MILLION times faster than Argon2\n' +
             'This is why password security uses deliberately SLOW algorithms!',
             ha='center', fontsize=13, fontweight='bold', color='#44ffff',
             bbox=dict(boxstyle='round,pad=1', facecolor='black', edgecolor='#44ffff', linewidth=2))
    
    plt.tight_layout(rect=[0, 0.06, 1, 1])
    plt.savefig(output_path, dpi=300, facecolor='#0a0a0a', edgecolor='none')
    print(f"📊 Saved: {output_path}")
    plt.close()

def plot_keyspace_visualization(output_path):
    """
    visualize how keyspace grows exponentially
    """
    fig, ax = plt.subplots(figsize=(14, 10))
    
    lengths = range(1, 17)
    lowercase_only = [26**l for l in lengths]
    lowercase_digits = [36**l for l in lengths]
    all_chars = [62**l for l in lengths]  # a-z, A-Z, 0-9
    all_plus_special = [95**l for l in lengths]  # + special chars
    
    ax.plot(lengths, lowercase_only, 'o-', linewidth=3, markersize=8, 
            label='Lowercase only (26)', color='#ff4444')
    ax.plot(lengths, lowercase_digits, 's-', linewidth=3, markersize=8,
            label='Lowercase + digits (36)', color='#ffaa44')
    ax.plot(lengths, all_chars, '^-', linewidth=3, markersize=8,
            label='Alphanumeric (62)', color='#44ff44')
    ax.plot(lengths, all_plus_special, 'd-', linewidth=3, markersize=8,
            label='+ Special chars (95)', color='#4444ff')
    
    ax.set_yscale('log')
    ax.set_xlabel('Password Length', fontsize=16, fontweight='bold')
    ax.set_ylabel('Total Possible Passwords (log scale)', fontsize=16, fontweight='bold')
    ax.set_title('🔢 Password Keyspace Growth\nWhy Length Matters More Than Complexity', 
                 fontsize=20, fontweight='bold', pad=20)
    
    ax.legend(fontsize=13, loc='upper left', framealpha=0.9)
    ax.grid(True, alpha=0.3)
    
    # add annotations for key lengths
    for l in [8, 12, 16]:
        idx = l - 1
        y_val = lowercase_digits[idx]
        ax.annotate(f'{l} chars:\n{y_val:.2e}',
                   xy=(l, y_val), xytext=(l+0.5, y_val*10),
                   fontsize=10, fontweight='bold', color='white',
                   bbox=dict(boxstyle='round,pad=0.5', facecolor='black', alpha=0.8),
                   arrowprops=dict(arrowstyle='->', color='white', lw=2))
    
    fig.text(0.5, 0.02,
             '📏 Every additional character multiplies security exponentially!\n' +
             'A 12-char password has 4,738,381,338,321,616,896 possible combinations',
             ha='center', fontsize=12, fontweight='bold', color='#ffff44',
             bbox=dict(boxstyle='round,pad=1', facecolor='black', edgecolor='#ffff44', linewidth=2))
    
    plt.tight_layout(rect=[0, 0.06, 1, 1])
    plt.savefig(output_path, dpi=300, facecolor='#0a0a0a', edgecolor='none')
    print(f"📊 Saved: {output_path}")
    plt.close()

def plot_real_world_scenarios(hash_rate, output_path):
    """
    show real-world cracking scenarios
    """
    fig, ax = plt.subplots(figsize=(14, 10))
    
    scenarios = [
        ('Common word\n(dictionary)', 0.001, '#ff0000'),
        ('Common + numbers\n("password123")', 0.005, '#ff4444'),
        ('8 char lowercase\n(weak)', 36**8/hash_rate, '#ff8844'),
        ('8 char mixed\n(okay)', 62**8/hash_rate, '#ffaa44'),
        ('10 char mixed\n(good)', 62**10/hash_rate, '#ffcc44'),
        ('12 char mixed\n(strong)', 62**12/hash_rate, '#44ff44'),
        ('14 char mixed\n(excellent)', 62**14/hash_rate, '#00ff00'),
        ('16 char mixed\n(fortress)', 62**16/hash_rate, '#00ff88'),
    ]
    
    names = [s[0] for s in scenarios]
    times = [s[1] for s in scenarios]
    colors = [s[2] for s in scenarios]
    
    bars = ax.barh(names, times, color=colors, edgecolor='white', linewidth=2)
    ax.set_xscale('log')
    ax.set_xlabel('Time to Crack (seconds, log scale)', fontsize=16, fontweight='bold')
    ax.set_title(f'⏱️  Real-World Cracking Times\n@ {hash_rate/1e9:.1f} Billion Hashes/Second', 
                 fontsize=20, fontweight='bold', pad=20)
    
    # add time labels
    for bar, time in zip(bars, times):
        label = format_time(time)
        ax.text(bar.get_width()*2, bar.get_y() + bar.get_height()/2,
                label, va='center', fontsize=12, 
                fontweight='bold', color='white',
                bbox=dict(boxstyle='round,pad=0.4', facecolor='black', alpha=0.8))
    
    # add safety zones
    ax.axvline(x=1, color='#ff0000', linestyle='--', linewidth=3, alpha=0.7, label='INSTANT')
    ax.axvline(x=3600, color='#ffaa00', linestyle='--', linewidth=3, alpha=0.7, label='< 1 hour')
    ax.axvline(x=86400, color='#ffff00', linestyle='--', linewidth=3, alpha=0.7, label='< 1 day')
    ax.axvline(x=31536000, color='#00ff00', linestyle='--', linewidth=3, alpha=0.7, label='< 1 year')
    
    ax.legend(loc='lower right', fontsize=12, framealpha=0.9)
    ax.grid(True, alpha=0.3, axis='x')
    
    fig.text(0.5, 0.02,
             '🎯 Use 12+ characters with mixed case, numbers, and symbols!\n' +
             'Enable 2FA for extra protection!',
             ha='center', fontsize=13, fontweight='bold', color='#00ff00',
             bbox=dict(boxstyle='round,pad=1', facecolor='black', edgecolor='#00ff00', linewidth=3))
    
    plt.tight_layout(rect=[0, 0.06, 1, 1])
    plt.savefig(output_path, dpi=300, facecolor='#0a0a0a', edgecolor='none')
    print(f"📊 Saved: {output_path}")
    plt.close()

def main():
    """generate all visualizations"""
    print("🔥 BlitzForge Password Cracking Visualizer\n")
    
    # estimated hash rate (adjust based on your actual benchmarks)
    # default: 5 GH/s (5 billion hashes per second)
    hash_rate = 5e9
    
    if len(sys.argv) > 1:
        try:
            hash_rate = float(sys.argv[1]) * 1e9
            print(f"Using custom hash rate: {hash_rate/1e9:.1f} GH/s")
        except:
            print(f"Using default hash rate: {hash_rate/1e9:.1f} GH/s")
    else:
        print(f"Using default hash rate: {hash_rate/1e9:.1f} GH/s")
        print("(Run with: python visualize_results.py <GH/s> to customize)\n")
    
    # create output directory
    output_dir = Path('graphs')
    output_dir.mkdir(exist_ok=True)
    
    print("\n📊 Generating visualizations...\n")
    
    # generate all plots
    plot_crack_time_by_length(hash_rate, output_dir / 'crack_time_by_length.png')
    plot_password_strength_comparison(hash_rate, output_dir / 'weak_vs_strong.png')
    plot_algorithm_comparison(hash_rate, output_dir / 'algorithm_comparison.png')
    plot_keyspace_visualization(output_dir / 'keyspace_growth.png')
    plot_real_world_scenarios(hash_rate, output_dir / 'real_world_times.png')
    
    print("\n✅ All visualizations generated in ./graphs/")
    print("\n📁 Generated files:")
    print("   • crack_time_by_length.png  - Time vs password length")
    print("   • weak_vs_strong.png        - Weak vs strong comparison")
    print("   • algorithm_comparison.png  - Algorithm speed comparison")
    print("   • keyspace_growth.png       - Exponential keyspace growth")
    print("   • real_world_times.png      - Real-world cracking scenarios")
    print("\n🎨 Use these in your presentation to show PASSWORD CRACKING POWER!")

if __name__ == '__main__':
    main()