addpath(".");

close all
clear
clc
format long g

%% Parameters
num_objectives = 3;

runs = 30;

root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "data", "Mutation");
out_folder = fullfile(root_path, "processed", "mutation");

sizes = [12, 16, 20, 24, 28, 32];
mutparam = [1, 2, 3, 4, 5, 10, 20, 50, 100];
ops = ["ca", "std"];
algs = ["IBEA", "MOEAD", "NSGAII"];

max_run = 30;

if ~exist(out_folder)
    mkdir(out_folder);
end

for alg = algs
    for op = ops
        output = [];
        
        for i = 1:length(sizes)
            size = sizes(i);
            
            best_hv = 0;
            best = 1;
            
            hvs = zeros(length(mutparam), max_run-1);
            times = zeros(length(mutparam), max_run-1);
            
            for j = 1 : length(mutparam)
                mut = mutparam(j);
                
                for k = 0 : max_run - 1
                    s = num2str(size);
                    p = num2str(mut);
                    r = num2str(k);
                    
                    folder = fullfile(src_folder, s, p, r, op, alg);
                    
                    hv_file = fullfile(folder, "HV.out");
                    time_file = fullfile(folder, "time.out");
                    
                    hv = csvread(hv_file);
                    hv = hv(2);
                    time = csvread(time_file);
                    
                    hvs(j, k+1) = hv;
                    times(j, k+1) = time;
                end
                
                mean_hv = mean(hvs(j, :));
                mean_time = mean(times(j, :));
                
                if mean_hv > best_hv
                    best_hv = mean_hv;
                    best = j;
                end
            end
            
            for j = 1 : length(mutparam)
                mean_hv = mean(hvs(j, :));
                mean_time = mean(times(j, :));
                
                [sig_test, h] = ranksum(hvs(best, :), hvs(j, :), 'tail', 'right');
                
                output{i, j} = [mean_hv, h];
            end
        end
        
        out_file = fullfile(out_folder, strcat(alg, "_", upper(op), ".tbl"));
        fid = fopen(out_file, 'w');
        
        for i = 1:length(sizes)
            fprintf(fid, '%i & ', sizes(i));
            
            for j = 1:length(mutparam)            
                cell = output{i, j};
                fprintf(fid, '%f & %i ', cell(1), cell(2));
                
                if j < length(mutparam) - 1
                    fprintf(fid, '& ');
                end
            end
            
            fprintf(fid, '\\\\ \n');
        end
        
        fclose(fid);
    end
end